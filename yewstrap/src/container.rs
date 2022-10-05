use yew::{html, Children, Component, Properties};

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    pub fluid: bool,
    pub children: Children,
}

pub struct Container;
impl Component for Container {
    type Message = ();

    type Properties = ContainerProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let class = if ctx.props().fluid {
            "container-fluid"
        } else {
            "container"
        };

        html!(
            <div class={class}>{ctx.props().children.clone()}</div>
        )
    }
}
