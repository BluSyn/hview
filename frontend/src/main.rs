use anyhow::Error;
use serde::Deserialize;
use std::rc::Rc;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::ConsoleService;
use yew::Properties;

enum EntryMsg {
    Display,
    // Delete
    // Rename
}

#[derive(Properties, Deserialize, Debug, Clone, PartialEq)]
struct EntryProps {
    name: String,
    path: String,
    size: u64,
    date: u64,
    date_string: String,
    thumb: Option<String>,
    ext: Option<String>,
}

struct Entry {
    props: EntryProps,
    link: ComponentLink<Self>,
}

impl Component for Entry {
    type Message = EntryMsg;
    type Properties = EntryProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::Display => true,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let p = &self.props;
        let thumb = if let Some(thumb) = &p.thumb {
            html! {
                <a href={ p.path.clone() } class="file">
                    <img src={ thumb.clone() } loading="lazy" class="thumb pb-3" />
                </a>
            }
        } else {
            html! {}
        };

        html! {
            <div>
                { thumb }
                <a href={ p.path.clone() } class="file file-link">
                    <i class="bi bi-file-richtext text-success"></i>
                    <strong>{ &p.name }</strong>
                </a>
                <br />
                <small>{ &p.size }{ "B" }</small> { "/" }
                <small><time datetime={ p.date_string.clone() }>{ &p.date_string }</time></small>
            </div>
        }
    }
}

pub enum MediaType {
    Image,
    Video,
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
                self.task = self.fetch_page();
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
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.update(AppMsg::Init);
        }
    }

    fn view(&self) -> Html {
        let content = if self.data.is_none() {
            html! {
                <>
                    <p>{ "Loading.." }</p>
                </>
            }
        } else {
            let data = self.data.as_ref().unwrap();
            let folders = data.folders.iter().map(|e| {
                html! {
                    <Entry
                    name={e.name.clone()}
                    path={e.path.clone()}
                    size={e.size.clone()}
                    date={e.date.clone()}
                    date_string={e.date_string.clone()}
                    thumb={e.thumb.clone()}
                    ext={e.ext.clone()} />
                }
            });
            html! {
                <section>
                { for folders }
                </section>
            }
        };

        html! {
            <>
                <main>
                    { content }
                </main>
            </>
        }
    }
}

impl App {
    fn fetch_page(&self) -> Option<FetchTask> {
        let request = Request::get("http://localhost:8000/")
            .header("Sec-Fetch-Mode", "no-cors")
            .header("Sec-Fetch-Site", "cross-site")
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
