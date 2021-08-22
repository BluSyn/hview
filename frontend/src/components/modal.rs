use yew::prelude::*;
use crate::MediaType;

enum ModalMsg {
    Show,
    Hide,
    Next,
    Previous
};
struct ModalProps {
    src: String,
    type: Mediatype
}
impl Component for Modal {
    type Message = ModalMsg;
    type Properties = ModalProps;

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
                      <video controls />
                  </div>
              </div>
            </div>
          </div>
        </div>
        }
    }
}
