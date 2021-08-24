use crate::{AppAnchor, AppRoute, SERVER_URL};
use serde::Deserialize;
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew::Properties;

pub enum EntryMsg {
    // Display,
    // Delete,
    // Rename,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub enum EntryType {
    File,
    Folder,
}

impl Default for EntryType {
    fn default() -> Self {
        Self::File
    }
}

impl Into<EntryType> for &str {
    fn into(self) -> EntryType {
        match self {
            "file" => EntryType::File,
            "folder" => EntryType::Folder,
            _ => EntryType::File,
        }
    }
}
impl Into<&str> for EntryType {
    fn into(self) -> &'static str {
        match self {
            EntryType::File => "file",
            EntryType::Folder => "folder",
        }
    }
}
impl IntoPropValue<EntryType> for &str {
    fn into_prop_value(self) -> EntryType {
        self.into()
    }
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
    #[prop_or_default]
    #[serde(skip)]
    pub etype: EntryType,
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

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        // match msg {
        //     EntryMsg::Display => true,
        //     EntryMsg::Delete => true,
        //     EntryMsg::Rename => true,
        // }
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
        let etype: &str = p.etype.to_owned().into();

        // Ensure paths have proper trailing /
        let link = match &p.etype {
            EntryType::File => format!("/{}", &p.path),
            EntryType::Folder => format!("/{}/", &p.path),
        };

        let thumb = if let Some(thumb) = &p.thumb {
            let src = format!("{}{}", SERVER_URL, &thumb);
            html! {
            <>
                <AppAnchor classes={ etype } route=AppRoute::Entry(link.to_owned())>
                    <img src={ src } loading="lazy" class="thumb pb-3" />
                </AppAnchor><br />
            </>
            }
        } else {
            html! {}
        };

        let icon = match &p.etype {
            EntryType::File => classes!("bi", "bi-file-richtext", "text-success"),
            EntryType::Folder => classes!("bi", "bi-folder-fill", "text-info"),
        };

        html! {
            <section class=classes!("col-xs-12","col-sm-6","col-md-4","col-lg-3","mb-sm-2","mb-lg-5","text-break", etype)>
                { thumb }
                <AppAnchor classes={ etype } route=AppRoute::Entry(link)>
                    <i class={ icon }></i>
                    <strong>{" "}{ &p.name }</strong>
                </AppAnchor><br />
                <small>{ &p.size }{ "B" }</small> { " / " }
                <small><time datetime={ p.date_string.to_owned() }>{ &p.date_string }</time></small>
            </section>
        }
    }
}
