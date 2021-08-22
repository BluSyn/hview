use serde::Deserialize;
use yew::prelude::*;
use yew::Properties;

pub enum EntryMsg {
    Display,
    Delete,
    Rename,
}

#[derive(Properties, Deserialize, Debug, Clone, PartialEq)]
pub struct EntryProps {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub date: u64,
    pub date_string: String,
    pub thumb: Option<String>,
    pub ext: Option<String>,
}

pub struct Entry {
    pub props: EntryProps,
    pub link: ComponentLink<Self>,
}

impl Component for Entry {
    type Message = EntryMsg;
    type Properties = EntryProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            EntryMsg::Display => true,
            EntryMsg::Delete => true,
            EntryMsg::Rename => true,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let p = &self.props;
        let thumb = if let Some(thumb) = &p.thumb {
            html! {
                <a href={ p.path.to_owned() } class="file">
                    <img src={ thumb.to_owned() } loading="lazy" class="thumb pb-3" />
                </a>
            }
        } else {
            html! {}
        };

        html! {
            <div>
                { thumb }
                <a href={ p.path.to_owned() } class="file file-link">
                    <i class="bi bi-file-richtext text-success"></i>
                    <strong>{ p.name.to_owned() }</strong>
                </a>
                <br />
                <small>{ p.size.to_owned() }{ "B" }</small> { " / " }
                <small><time datetime={ p.date_string.to_owned() }>{ p.date_string.to_owned() }</time></small>
            </div>
        }
    }
}
