use yew::{html, Children, Component, Properties};

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    pub children: Children,
    #[prop_or_default]
    pub hidden: bool
}

pub struct Container;
impl Component for Container {
    type Message = ();

    type Properties = ContainerProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html!(
            <div hidden={ctx.props().hidden} class="container mt-2">{ctx.props().children.clone()}</div>
        )
    }
}
