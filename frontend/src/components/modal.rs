use yew::html::IntoPropValue;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::Properties;

use super::page::{Page, PageMsg};
use crate::{App, SERVER_URL};
use wasm_bindgen::prelude::*;
use web_sys::Element;

// Modal uses external Bootstrap Modal
// TODO: In the future this could be brought "in-house"
// however this works as a good proof-of-concept
#[wasm_bindgen]
extern "C" {
    pub type BootstrapModal;

    #[wasm_bindgen(js_namespace = bootstrap, js_class = Modal, constructor)]
    fn new(element: Element) -> BootstrapModal;

    #[wasm_bindgen(method)]
    fn show(this: &BootstrapModal) -> bool;

    #[wasm_bindgen(method)]
    fn hide(this: &BootstrapModal) -> bool;
}

pub const IMG_TYPES: [&str; 8] = [
    ".jpg", ".jpeg", ".jpe", ".png", ".gif", ".avif", ".webp", ".heic",
];
pub const VID_TYPES: [&str; 9] = [
    ".mp4", ".webm", ".mts", ".mov", ".ogv", ".ogg", ".mp3", ".flac", ".wav",
];
pub fn is_img(path: &str) -> bool {
    let p = &path.to_lowercase();
    IMG_TYPES.iter().find(|&&t| p.contains(t)).is_some()
}

pub fn is_vid(path: &str) -> bool {
    let p = &path.to_lowercase();
    VID_TYPES.iter().find(|&&t| p.contains(t)).is_some()
}

pub fn is_media(path: &str) -> bool {
    is_img(&path) || is_vid(&path)
}

#[derive(Clone, Debug, PartialEq)]
pub enum MediaType {
    Image,
    Video,
    None,
}

impl MediaType {
    pub fn from_path(media: &str) -> Self {
        if media == "" {
            Self::None
        } else if is_img(media) {
            Self::Image
        } else if is_vid(media) {
            Self::Video
        } else {
            Self::None
        }
    }
}

impl IntoPropValue<MediaType> for &str {
    fn into_prop_value(self) -> MediaType {
        match self {
            "image" => MediaType::Image,
            "video" => MediaType::Video,
            _ => MediaType::None,
        }
    }
}

impl Into<&str> for MediaType {
    fn into(self) -> &'static str {
        match self {
            MediaType::Image => "image",
            MediaType::Video => "video",
            MediaType::None => "none",
        }
    }
}

pub enum ModalMsg {
    // Show,
    Hide,
    Next,
    Previous,
    None,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct ModalProps {
    pub src: String,
    pub media: MediaType,
}

impl Default for ModalProps {
    fn default() -> ModalProps {
        ModalProps {
            src: String::from(""),
            media: MediaType::None,
        }
    }
}

pub struct Modal {
    pub link: ComponentLink<Self>,
    pub props: ModalProps,
    pub instance: Option<BootstrapModal>,
}

impl Component for Modal {
    type Message = ModalMsg;
    type Properties = ModalProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            instance: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ModalMsg::Next => {
                self.link
                    .get_parent()
                    .expect("Parent Comp")
                    .clone()
                    .downcast::<Page>()
                    .send_message(PageMsg::ModalNext);
            }
            ModalMsg::Previous => {
                self.link
                    .get_parent()
                    .expect("Parent Comp")
                    .clone()
                    .downcast::<Page>()
                    .send_message(PageMsg::ModalPrevious);
            }
            ModalMsg::Hide => {
                // Hide by navigating to parent directory
                if let Some(index) = &self.props.src.rfind('/') {
                    let path = &self.props.src[0..index + 1];
                    App::change_route(path.to_string());
                }
            }
            _ => {}
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let p = &self.props;
        ConsoleService::info(format!("Rendering Modal: {:?}", p.src).as_str());

        let src = format!("{}{}", SERVER_URL, p.src);
        let media = match p.media {
            MediaType::Image => {
                let bg = format!("background-image:url('{}')", src);
                html! {
                  <div id="media_img" style={ bg }></div>
                }
            }
            MediaType::Video => {
                html! {
                  <div id="media_vid">
                      <video controls=true src={ src } />
                  </div>
                }
            }
            MediaType::None => {
                html! {}
            }
        };

        let onkeydown =
            self.link
                .callback(|event: web_sys::KeyboardEvent| match event.key().as_str() {
                    "Escape" => ModalMsg::Hide,
                    "ArrowRight" => ModalMsg::Next,
                    "ArrowLeft" => ModalMsg::Previous,
                    "Backspace" => ModalMsg::Previous,
                    " " => ModalMsg::Next,
                    _ => ModalMsg::None,
                });

        html! {
            <div class="modal fade" id="media_modal" tabindex="-1" aria-hidden="true" data-bs-keyboard="false" onkeydown={ onkeydown }>
              <div class="modal-dialog modal-fullscreen">
                <div class="modal-content">
                  <div class="modal-body">{ media }</div>
                </div>
              </div>
            </div>
        }
    }

    // Handle Bootstrap Modal instance
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let element = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("media_modal")
                .unwrap();
            self.instance = Some(BootstrapModal::new(element));
        } else if self.props.media != MediaType::None {
            self.instance.as_ref().unwrap().show();
            ConsoleService::info("Modal Show");
        } else {
            self.instance.as_ref().unwrap().hide();
            ConsoleService::info("Modal Hide");
        }
    }
}
