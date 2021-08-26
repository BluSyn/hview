use yew::prelude::*;
use yew::services::ConsoleService;
use yew::Properties;
use yew_router::components::RouterAnchor;
use yew_router::prelude::*;

mod components;
use crate::components::{
    modal::{MediaType, Modal},
    page::Page,
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
