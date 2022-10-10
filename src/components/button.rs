use yew::html::onclick::Event;
use yew::{callback::Callback, html, Children, Component, Properties};
#[allow(dead_code)] // TODO remove when start using!
#[derive(PartialEq,Eq)]
pub enum Style {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
    Link,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub children: Children,
    pub onclick: Callback<Event>,
    pub style: Option<Style>,
    pub block: Option<bool>,
}

pub struct Button;
impl Component for Button {
    type Message = ();

    type Properties = ButtonProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        println!("Button created");
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let style = match &ctx.props().style {
            Some(style) => match style {
                Style::Primary => "btn-primary",
                Style::Secondary => "btn-secondary",
                Style::Success => "btn-success",
                Style::Danger => "btn-danger",
                Style::Warning => "btn-warning",
                Style::Info => "btn-info",
                Style::Light => "btn-light",
                Style::Dark => "btn-dark",
                Style::Link => "btn-link",
            },
            None => "btn-primary",
        };
        let classes = format!("btn {style} btn-lg");
        let inner = ctx.props().children.clone();
        let onclick = &ctx.props().onclick;
        if let Some(true) = ctx.props().block {
            html!(
                <div class="d-grid">
                    <button onclick={onclick} class={classes}>{inner}</button>
                </div>
            )
        } else {
            html!(
                <button onclick={onclick} class={classes}>{inner}</button>
            )
        }
    }
}
