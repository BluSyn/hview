use std::fmt;
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew::Properties;

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
    Show,
    Hide,
    Next,
    Previous,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct ModalProps {
    pub src: String,
    pub media: MediaType,
}
pub struct Modal {
    pub link: ComponentLink<Self>,
    pub props: ModalProps,
}
impl Component for Modal {
    type Message = ModalMsg;
    type Properties = ModalProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let p = &self.props;
        let media = match p.media {
            MediaType::Image => {
                html! {
                  <div id="media_img">
                      <img draggable="false" title="" src={ p.src.to_owned() } />
                  </div>
                }
            }
            MediaType::Video => {
                html! {
                  <div id="media_vid">
                      <video controls=true src={ p.src.to_owned() } />
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
}
