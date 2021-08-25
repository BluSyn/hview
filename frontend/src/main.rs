use serde::Deserialize;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::ConsoleService;
use yew::Properties;
use yew_router::components::RouterAnchor;
use yew_router::prelude::*;

mod components;
use crate::components::{
    entry::{Entry, EntryProps},
    modal::{MediaType, Modal},
};

// TODO: Move this to config
pub const SERVER_URL: &str = "http://localhost:8000/";

pub const IMG_TYPES: [&str; 8] = [
    ".jpg", ".jpeg", ".jpe", ".png", ".gif", ".avif", ".webp", ".heic",
];
pub fn is_img(path: &str) -> bool {
    let p = &path.to_lowercase();
    IMG_TYPES.iter().find(|&&t| p.contains(t)).is_some()
}

pub const VID_TYPES: [&str; 9] = [
    ".mp4", ".webm", ".mts", ".mov", ".ogv", ".ogg", ".mp3", ".flac", ".wav",
];
pub fn is_vid(path: &str) -> bool {
    let p = &path.to_lowercase();
    VID_TYPES.iter().find(|&&t| p.contains(t)).is_some()
}

pub fn is_media(path: &str) -> bool {
    is_img(&path) || is_vid(&path)
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
}

#[derive(Properties, Clone, PartialEq)]
pub struct PageProps {
    path: String,
    page: Option<Dir>,
    callback: Option<Callback<String>>,
}

pub struct Page {
    link: ComponentLink<Self>,
    props: PageProps,
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
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.path != props.path {
            if is_media(props.path.as_str()) {
                // Trigger modal
                self.props
                    .callback
                    .as_ref()
                    .unwrap()
                    .emit(props.path.to_owned());
            } else {
                ConsoleService::info(format!("Page Changed: {:?}", props.path).as_str());
                self.task = self.fetch_page(props.path.as_str());

                // Reset Modal
                self.props.callback.as_ref().unwrap().emit("".to_string());
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
                self.props
                    .callback
                    .as_ref()
                    .unwrap()
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

pub enum AppMsg {
    LoadModal(String),
}
#[derive(Properties, Clone, Debug, PartialEq)]
pub struct AppProps {
    modal_src: String,
    modal_type: MediaType,
}
impl Default for AppProps {
    fn default() -> AppProps {
        AppProps {
            modal_src: String::from(""),
            modal_type: MediaType::Image,
        }
    }
}
pub struct App {
    props: AppProps,
    link: ComponentLink<Self>,
}
impl Component for App {
    type Message = AppMsg;
    type Properties = AppProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMsg::LoadModal(src) => {
                ConsoleService::info(format!("Loading modal for: {:?}", src).as_str());
                self.props.modal_src = src.to_string();

                // Handle default case (if string is empty) first
                self.props.modal_type = if src == String::from("") {
                    MediaType::None
                } else if is_img(&src) {
                    MediaType::Image
                } else if is_vid(&src) {
                    MediaType::Video
                } else {
                    MediaType::None
                };

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cb = self.link.callback(AppMsg::LoadModal);
        let render = Router::render(move |switch: AppRoute| -> Html {
            match switch {
                AppRoute::Entry(path) => {
                    html! { <Page path={ path } callback={ &cb } /> }
                }
            }
        });
        html! {
            <>
                <Modal src={ self.props.modal_src.to_owned() } media={ self.props.modal_type.to_owned() } />
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
