use serde::Deserialize;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::ConsoleService;
use yew::Properties;

use web_sys::Node;
use yew::virtual_dom::VNode;

use super::entry::{Entry, EntryProps};
use super::modal::{is_media, MediaType, Modal, ModalProps};
use crate::{App, SERVER_URL};

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
    PageLoad(Result<Dir, anyhow::Error>),
    LoadModal(String),
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
    task: Option<FetchTask>,
    modal: ModalProps,
}

impl Component for Page {
    type Message = PageMsg;
    type Properties = PageProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            task: None,
            modal: ModalProps::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PageMsg::PageLoad(result) => match result {
                Ok(data) => {
                    self.props.page = Some(data);
                    true
                }
                Err(error) => {
                    ConsoleService::error(format!("Invalid response: {:?}", error).as_str());
                    false
                }
            },
            PageMsg::LoadModal(src) => {
                ConsoleService::info(format!("Loading modal for: {:?}", src).as_str());
                self.modal.src = src.to_string();
                self.modal.media = MediaType::new(src.as_str());

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
            if is_media(props.path.as_str()) {
                // Trigger modal
                self.link
                    .callback(PageMsg::LoadModal)
                    .emit(props.path.to_owned());
            } else {
                ConsoleService::info(format!("Page Changed: {:?}", props.path).as_str());
                self.task = self.fetch_page(props.path.as_str());

                // Reset Modal
                self.modal = ModalProps::default()
            }

            self.props.path = props.path;
            // TODO: Render here to show loading animation?
            false
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
                    .callback(PageMsg::LoadModal)
                    .emit(self.props.path.to_owned());

                // Get dir of file
                let index = self.props.path.rfind('/').unwrap();
                fetch_path = &self.props.path[0..index + 1];
            } else {
                fetch_path = &self.props.path;
            }

            self.task = self.fetch_page(fetch_path);
        }
    }

    fn view(&self) -> Html {
        let mut title = "Loading...";
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
            html! { <p>{ "..." }</p> }
        };

        // Convert title into span's for each subdir
        // TODO: This is a hack to inject HTML directly into macro
        // is there a better solution?
        let html_title = {
            let combined = format!("{}{}", base_path, title);
            let split = combined.split_inclusive('/').enumerate();
            let clone = split.clone();
            let span = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("h1")
                .unwrap();
            span.set_id("title");
            span.set_inner_html(
                split
                    .map(|s| {
                        // TODO: Moving this to separate function required
                        // an impossible to define type signature
                        format!(
                            "<a href={}>{}</a>",
                            clone
                                .clone()
                                .filter(|&(i, _)| i <= s.0)
                                .map(|(_, e)| e)
                                .collect::<String>(),
                            s.1
                        )
                    })
                    .collect::<String>()
                    .as_str(),
            );
            let node = Node::from(span);
            VNode::VRef(node)
        };

        html! {
            <>
                <Modal src={ self.modal.src.to_owned() } media={ self.modal.media.to_owned() } />
                { html_title }
                { content }
            </>
        }
    }
}

impl Page {
    fn fetch_page(&self, path: &str) -> Option<FetchTask> {
        let request = Request::get(format!("{}{}", SERVER_URL, path).as_str())
            .body(Nothing)
            .expect("Could not load from API");
        let callback =
            self.link
                .callback(|response: Response<Json<Result<Dir, anyhow::Error>>>| {
                    let Json(data) = response.into_body();
                    PageMsg::PageLoad(data)
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
