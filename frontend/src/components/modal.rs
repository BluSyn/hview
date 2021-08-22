use yew::prelude::*;
use yew::Properties;

#[derive(Clone, Debug, PartialEq)]
pub enum MediaType {
    Image,
    Video,
}

pub enum ModalMsg {
    Show,
    Hide,
    Next,
    Previous,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct ModalProps {
    src: String,
    media: MediaType,
}
pub struct Modal {
    pub link: ComponentLink<Self>,
}
impl Component for Modal {
    type Message = ModalMsg;
    type Properties = ModalProps;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
        <div class="modal fade" id="media_modal" tabindex="-1" aria-hidden="true">
          <div class="modal-dialog modal-fullscreen">
            <div class="modal-content">
              <div class="modal-body">
                  <div id="media_img" class="visually-hidden">
                      <img draggable="false" />
                  </div>
                  <div id="media_vid" class="visually-hidden">
                      <video controls=true />
                  </div>
              </div>
            </div>
          </div>
        </div>
        }
    }
}
