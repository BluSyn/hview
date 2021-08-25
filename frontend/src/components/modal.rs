use yew::html::IntoPropValue;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::Properties;

use wasm_bindgen::prelude::*;
use web_sys::Element;

use crate::SERVER_URL;

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
}

#[derive(Clone, Debug, PartialEq)]
pub enum MediaType {
    Image,
    Video,
}

impl IntoPropValue<MediaType> for &str {
    fn into_prop_value(self) -> MediaType {
        match self {
            "image" => MediaType::Image,
            "video" => MediaType::Video,
            _ => MediaType::Image,
        }
    }
}

impl Into<&str> for MediaType {
    fn into(self) -> &'static str {
        match self {
            MediaType::Image => "image",
            MediaType::Video => "video",
        }
    }
}

pub enum ModalMsg {
    // Show,
// Hide,
// Next,
// Previous,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct ModalProps {
    pub src: String,
    pub media: MediaType,
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

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
        };

        html! {
            <div class="modal fade" id="media_modal" tabindex="-1" aria-hidden="true">
              <div class="modal-dialog modal-fullscreen">
                <div class="modal-content">
                  <div class="modal-body">{ media }</div>
                </div>
              </div>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let modal_element = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("media_modal")
                .unwrap();
            self.instance = Some(BootstrapModal::new(modal_element));
        } else {
            self.instance.as_ref().unwrap().show();
        }
    }
}
