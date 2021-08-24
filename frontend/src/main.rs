use anyhow::Error;
use serde::Deserialize;
use url::Url;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::ConsoleService;
use yew::utils::document;
use yew::Properties;
use yew_router::components::RouterAnchor;
use yew_router::prelude::*;
// use yew_router::switch::Permissive;

mod components;
use crate::components::{
    entry::{Entry, EntryProps},
    modal::{Modal, ModalProps},
};

// TODO: Move this to config
pub const SERVER_URL: &str = "http://localhost:8000/";

pub const IMG_TYPES: &[&'static str] = &[
    ".jpg", ".jpeg", ".jpe", ".png", ".gif", ".avif", ".webp", ".heic",
];
pub fn is_img(path: &str) -> bool {
    let p = &path.to_lowercase();
    IMG_TYPES.iter().find(|&&t| p.contains(t)).is_some()
}

pub const VID_TYPES: &[&'static str] = &[
    ".mp4", ".webm", ".mts", ".mov", ".ogv", ".ogg", ".mp3", ".flac", ".wav",
];
pub fn is_vid(path: &str) -> bool {
    let p = &path.to_lowercase();
    VID_TYPES.iter().find(|&&t| p.contains(t)).is_some()
}

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
}

#[derive(Properties, Clone, PartialEq)]
pub struct PageProps {
    path: String,
    page: Option<Dir>,
}

pub struct Page {
    link: ComponentLink<Self>,
    props: PageProps,
    // data: Option<Dir>,
    task: Option<FetchTask>,
}

impl Component for Page {
    type Message = PageMsg;
    type Properties = PageProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            task: None,
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
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.path != props.path {
            if is_vid(props.path.as_str()) || is_img(props.path.as_str()) {
                let cb = self.link.callback(PageMsg::LoadModal);
                cb.emit(props.path.to_owned());
            } else {
                ConsoleService::info(format!("Page Changed: {:?}", props.path).as_str());
                self.task = self.fetch_page(props.path.as_str());
            }

            self.props.path = props.path;
            // TODO: Render here to show loading animation?
            false
        } else {
            false
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.task = self.fetch_page(self.props.path.as_str());
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

        html! {
            <>
                <h1 id="title">{ base_path }{ title }</h1>
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
}

pub struct App {}
impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(|switch: AppRoute| -> Html {
            match switch {
                AppRoute::Entry(path) => {
                    html! { <Page path={ path } /> }
                }
            }
        });
        html! {
            <>
                <Modal src="placeholder.png" media="image" />
                <main class="container">
                    <Router<AppRoute> render={ render } />
                </main>
            </>
        }
    }
}

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "{*}"]
    Entry(String),
}

pub type AppAnchor = RouterAnchor<AppRoute>;

fn main() {
    yew::start_app::<App>();
}
