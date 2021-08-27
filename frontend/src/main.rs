use yew::prelude::*;
use yew_router::components::RouterAnchor;
use yew_router::prelude::*;
use yew_router::{
    agent::{RouteAgent, RouteRequest},
    route::Route,
};

mod components;
use crate::components::page::Page;

// TODO: Move this to config
pub const SERVER_URL: &str = "http://localhost:8000";

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
}

fn main() {
    yew::start_app::<App>();
}
