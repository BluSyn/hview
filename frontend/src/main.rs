use anyhow::Error;
use serde::Deserialize;
use url::Url;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::ConsoleService;
use yew::utils::document;
use yew::Properties;
use yew_router::prelude::*;

mod components;
use crate::components::{
    entry::{Entry, EntryProps},
    modal::{Modal, ModalProps},
};

// TODO: Move this to config
pub const SERVER_URL: &str = "http://localhost:8000/";

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "{*}"]
    Entry(String),
}

impl AppRoute {
    pub fn into_router(self) -> Route {
        Route::from(self)
    }
}

#[derive(Deserialize, Debug)]
pub struct Dir {
    title: String,
    base_path: String,
    read_only: bool,
    files: Vec<EntryProps>,
    folders: Vec<EntryProps>,
}

#[derive(Debug)]
pub enum AppMsg {
    Init,
    PageLoad(Result<Dir, anyhow::Error>),
    LoadModal(String),
}
pub struct App {
    link: ComponentLink<Self>,
    data: Option<Dir>,
    task: Option<FetchTask>,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            data: None,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMsg::Init => {
                let current = document().url().unwrap();
                let url = Url::parse(current.as_str()).unwrap();
                self.task = self.fetch_page(url.path());
                true
            }
            AppMsg::PageLoad(result) => match result {
                Ok(data) => {
                    self.data = Some(data);
                    true
                }
                Err(error) => {
                    ConsoleService::error(format!("Invalid response: {:?}", error).as_str());
                    false
                }
            },
            AppMsg::LoadModal(src) => {
                ConsoleService::info(format!("Loading modal for: {:?}", src).as_str());
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(AppMsg::Init);
        }
    }

    fn view(&self) -> Html {
        let mut title = "";
        let mut base_path = "";

        let content = if self.data.is_none() {
            html! {
                <p>{ "Loading.." }</p>
            }
        } else {
            let data = self.data.as_ref().unwrap();
            title = data.title.as_str();
            base_path = data.base_path.as_str();

            let folders = data.folders.iter().map(|e| {
                html! {
                    <Entry with e.to_owned() />
                }
            });
            let files = data.files.iter().map(|e| {
                html! {
                    <Entry with e.to_owned() />
                }
            });
            html! {
                <div class="row gx-5">
                { for folders }
                { for files }
                </div>
            }
        };

        html! {
            <>
                <Modal src="placeholder.png" media="image" />
                <main class="container">
                    <h1 id="title">{ base_path }{ title }</h1>
                    { content }
                </main>
            </>
        }
    }
}

impl App {
    fn fetch_page(&self, path: &str) -> Option<FetchTask> {
        let request = Request::get(format!("{}{}", SERVER_URL, path).as_str())
            .body(Nothing)
            .expect("Could not load from API");
        let callback =
            self.link
                .callback(|response: Response<Json<Result<Dir, anyhow::Error>>>| {
                    let Json(data) = response.into_body();
                    AppMsg::PageLoad(data)
                });
        let task = FetchService::fetch(request, callback).expect("Could not load page");
        Some(task)
    }
}

fn main() {
    yew::start_app::<App>();
}
