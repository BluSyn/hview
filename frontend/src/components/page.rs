use serde::Deserialize;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::ConsoleService;
use yew::Properties;

use super::entry::{Entry, EntryProps};
use super::modal::{is_media, MediaType, Modal, ModalProps};
use crate::{App, AppAnchor, AppRoute, SERVER_URL};
use anyhow::{anyhow, Error};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Dir {
    title: String,
    base_path: String,
    read_only: bool,
    files: Vec<EntryProps>,
    folders: Vec<EntryProps>,
}

#[derive(Debug)]
pub enum PageMsg {
    Page(Dir),
    File,
    Error(Error),
    Modal(String),
    ModalNext,
    ModalPrevious,
}

#[derive(Properties, Clone, PartialEq)]
pub struct PageProps {
    pub path: String,
    pub page: Option<Dir>,
}

pub struct Page {
    link: ComponentLink<Self>,
    props: PageProps,
    modal: ModalProps,

    task: Option<FetchTask>,
    loaded: Option<String>,
    error: Option<Error>,
    show_loading: bool,
}

impl Component for Page {
    type Message = PageMsg;
    type Properties = PageProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            modal: ModalProps::default(),
            task: None,
            loaded: None,
            error: None,
            show_loading: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PageMsg::Page(page) => {
                self.props.page = Some(page);
                self.error = None;
                self.show_loading = false;
                true
            }
            // TODO: This means non-media (non-modal display) files
            // end up loading twice. Once on request,
            // and then again on popup. Parent directy also reloads.
            // Not sure the best solution at the moment.
            // (Start with HEAD request instead of GET?)
            PageMsg::File => {
                let url = format!("{}{}", *SERVER_URL, &self.props.path);
                web_sys::window()
                    .unwrap()
                    .open_with_url_and_target(&url, "_new_file")
                    .unwrap();

                self.show_loading = false;
                self.error = None;

                // Show containing dir by navigating to parent directory
                if let Some(index) = &self.props.path.rfind('/') {
                    let path = &self.props.path[0..index + 1];
                    App::replace_route(path.to_string());
                }

                false
            }
            PageMsg::Error(error) => {
                ConsoleService::error(format!("Invalid response: {:?}", error).as_str());
                self.error = Some(error);
                self.show_loading = false;
                self.modal = ModalProps::default();
                true
            }

            PageMsg::Modal(src) => {
                ConsoleService::info(format!("Loading modal for: {:?}", src).as_str());
                self.modal.src = src.to_string();
                self.modal.media = MediaType::from_path(src.as_str());
                self.show_loading = false;
                true
            }
            PageMsg::ModalNext => {
                let src = format!("/{}", self.next_file());
                App::change_route(src);
                true
            }
            PageMsg::ModalPrevious => {
                let src = format!("/{}", self.prev_file());
                App::change_route(src);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.path != props.path {
            ConsoleService::info(format!("Page Changed: {:?}", props.path).as_str());

            if is_media(props.path.as_str()) {
                // Trigger modal
                self.link
                    .callback(PageMsg::Modal)
                    .emit(props.path.to_owned());
                self.show_loading = true;
            } else {
                // Only re-fetch page if not already loaded
                if self.loaded.is_none() || self.loaded.as_ref().unwrap() != &props.path {
                    self.loaded = Some(props.path.to_owned());
                    self.task = self.fetch_page(props.path.as_str());
                    self.show_loading = true;
                } else {
                    self.show_loading = false;
                }

                // Reset Modal
                self.modal = ModalProps::default();
            }

            self.props.path = props.path;
            true
        } else {
            false
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // On page init, the path may be a dir or a file
        // display modal if it's a file + load directory the file is in
        if first_render {
            let fetch_path: &str;
            if is_media(&self.props.path.as_str()) {
                // Trigger modal
                self.link
                    .callback(PageMsg::Modal)
                    .emit(self.props.path.to_owned());

                // Get dir of file
                let index = self.props.path.rfind('/').unwrap();
                fetch_path = &self.props.path[0..index + 1];
            } else {
                fetch_path = &self.props.path;
            }

            self.loaded = Some(fetch_path.to_string());
            self.task = self.fetch_page(fetch_path);
        }
    }

    fn view(&self) -> Html {
        let mut title = "";
        let mut base_path = "";
        let content = if let Some(data) = &self.props.page {
            title = data.title.as_str();
            base_path = data.base_path.as_str();

            let folders = data.folders.iter().map(|e| {
                html! {
                    <Entry
                      name={ e.name.to_owned() }
                      path={ e.path.to_owned() }
                      size={ e.size.to_owned() }
                      date={ e.date.to_owned() }
                      date_string={ e.date_string.to_owned() }
                      thumb={ e.thumb.to_owned() }
                      ext={ e.ext.to_owned() }
                      etype="folder"
                      />
                }
            });
            let files = data.files.iter().map(|e| {
                html! {
                    <Entry
                      name={ e.name.to_owned() }
                      path={ e.path.to_owned() }
                      size={ e.size.to_owned() }
                      date={ e.date.to_owned() }
                      date_string={ e.date_string.to_owned() }
                      thumb={ e.thumb.to_owned() }
                      ext={ e.ext.to_owned() }
                      etype="file"
                      />
                }
            });
            html! {
                <div class="row gx-5">
                { for folders }
                { for files }
                </div>
            }
        } else {
            html! {}
        };

        // Convert title into links for each subdir
        let combined = if title == String::from("") {
            base_path.to_string()
        } else {
            format!("{}{}/", base_path, title)
        };
        let split = combined.split_inclusive('/').enumerate();
        let clone = split.clone();
        let html_title = split.map(|s| {
            // Note: Not happy with a loop of "clone" calls
            // but the strings have to be duplicated anyway.
            // Current solution adds one extra "clone" than is ideally necessary
            let link = clone
                .clone()
                .filter(|&(i, _)| i <= s.0)
                .map(|(_, e)| e)
                .collect::<String>();
            let text = &s.1;
            html! {
                <AppAnchor route={ AppRoute::Entry(link) }>{ text }</AppAnchor>
            }
        });

        let loading = if self.show_loading {
            html! {<span class="loading"></span>}
        } else {
            html! {}
        };

        let error = if self.error.is_some() {
            html! {<h2 class="text-danger">{ "Error: " }{ self.error.as_ref().unwrap() }</h2>}
        } else {
            html! {}
        };

        html! {
            <>
                <Modal src={ self.modal.src.to_owned() } media={ self.modal.media.to_owned() } />
                <h1 id="title">
                    { for html_title }
                    { loading }
                </h1>
                { error }
                { content }
            </>
        }
    }
}

impl Page {
    fn fetch_page(&self, path: &str) -> Option<FetchTask> {
        // TODO: This results in double "//" in path.
        // Not a major issue, but should be accounted for
        let url = format!("{}{}", *SERVER_URL, path);
        let request = Request::get(url.as_str())
            .body(Nothing)
            .expect("Could not load from API");
        let callback = self
            .link
            .callback(|response: Response<Json<Result<Dir, Error>>>| {
                let status = response.status();
                if !status.is_success() {
                    let err = anyhow!(
                        "Error: {} ({})",
                        &status.canonical_reason().unwrap(),
                        &status.as_str()
                    );
                    return PageMsg::Error(err);
                }

                let content = response.headers().get("content-type");
                if content.is_none() {
                    return PageMsg::Error(anyhow!("Invalid Content Type"));
                } else if content.unwrap() != &"application/json" {
                    return PageMsg::File;
                }

                let Json(data) = response.into_body();
                match data {
                    Ok(dir) => PageMsg::Page(dir),
                    Err(err) => PageMsg::Error(err),
                }
            });
        let task = FetchService::fetch(request, callback).expect("Could not load page");
        Some(task)
    }

    // Determine the next file in modal sequence
    fn next_file(&self) -> String {
        let findex = &self.modal.src.rfind('/').expect("complete path");
        let srcname = &self.modal.src[*findex + 1..];
        let page = &self.props.page.as_ref().unwrap();
        let files = &page.files;
        let current = files.iter().position(|e| e.name == srcname);
        if let Some(index) = current {
            if index + 1 >= files.len() {
                files.first().unwrap().path.to_owned()
            } else {
                files.get(index + 1).unwrap().path.to_owned()
            }
        } else {
            "".to_string()
        }
    }

    // Determine the prev file in modal sequence
    fn prev_file(&self) -> String {
        let findex = &self.modal.src.rfind('/').expect("complete path");
        let srcname = &self.modal.src[*findex + 1..];
        let page = &self.props.page.as_ref().unwrap();
        let files = &page.files;
        let current = files.iter().position(|e| e.name == srcname);
        if let Some(index) = current {
            if (index as i8) - 1 < 0 {
                files.last().unwrap().path.to_owned()
            } else {
                files.get(index - 1).unwrap().path.to_owned()
            }
        } else {
            "".to_string()
        }
    }
}
