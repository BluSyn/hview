use yew::html::IntoPropValue;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::Properties;

use crate::SERVER_URL;
use crate::{App, AppMsg};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

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

#[derive(Clone, Debug, PartialEq)]
pub enum MediaType {
    Image,
    Video,
    None,
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
    Show,
    Hide,
    Next,
    Previous,
    None,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct ModalProps {
    pub src: String,
    pub media: MediaType,
    pub callback: Option<Callback<()>>,
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
                    .downcast::<App>()
                    .send_message(AppMsg::LoadModal("placeholder.png".to_string()));
            }
            ModalMsg::Hide => {
                // HIde by calling parent callback?
                // self.props.callback.as_ref().unwrap().emit(());

                // Hide by navigating to parent directory
                let window = web_sys::window().expect("DOM Window");
                let index = &self.props.src.rfind('/').expect("complete path");
                let path = &self.props.src[0..index + 1];
                window
                    .history()
                    .expect("no history")
                    .push_state_with_url(&JsValue::NULL, "", Some(&path))
                    .expect("push history");
                let event = web_sys::PopStateEvent::new("popstate").expect("popstate event");
                window.dispatch_event(&event).expect("dispatch");

                ConsoleService::info(format!("Modal Closed: {:?}", path).as_str());
            }
            _ => {}
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;

            // If updated type is none, and modal instance already exists
            // signal to hide the modal
            if self.props.media == MediaType::None && self.instance.is_some() {
                self.instance.as_ref().unwrap().hide();
                false
            } else {
                true
            }
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
                html! {
                  <div id="media_img">
                      <img draggable="false" title="" src={ src } />
                  </div>
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
        } else {
            self.instance.as_ref().unwrap().show();
        }
    }
}
