use lazy_static::lazy_static;
use std::env;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew_router::components::RouterAnchor;
use yew_router::prelude::*;
use yew_router::{
    agent::{RouteAgent, RouteRequest},
    route::Route,
};

mod components;
use crate::components::page::Page;

// TODO: Move this to config?
lazy_static! {
    pub static ref SERVER_URL: String =
        env::var("TRUNK_PROXY_REWRITE").unwrap_or(String::from("/api/"));
}

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "{*}"]
    Entry(String),
}

pub type AppAnchor = RouterAnchor<AppRoute>;

pub struct App {}
impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(move |switch: AppRoute| -> Html {
            match switch {
                AppRoute::Entry(path) => {
                    ConsoleService::info("Loading Path");
                    html! { <Page path={ path } /> }
                }
            }
        });

        html! {
            <main class="container">
                <Router<AppRoute> render={ render } />
            </main>
        }
    }
}

impl App {
    pub fn change_route(src: String) {
        RouteAgent::dispatcher().send(RouteRequest::ChangeRoute(Route {
            route: src,
            state: (),
        }));
    }
    pub fn replace_route(src: String) {
        RouteAgent::dispatcher().send(RouteRequest::ReplaceRoute(Route {
            route: src,
            state: (),
        }));
    }
}

fn main() {
    yew::start_app::<App>();
}
