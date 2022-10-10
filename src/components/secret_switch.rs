use yew::prelude::*;

use super::app::SecretPackage;

pub struct SecretSwitchMessage;

#[derive(Properties, PartialEq)]
pub struct SecretSwitchProps {
    pub secret: SecretPackage,
}
pub struct SecretSwitch;

impl Component for SecretSwitch {
    type Message = SecretSwitchMessage;

    type Properties = SecretSwitchProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, _: Self::Message) -> bool {
        ctx.props().secret.toggler.emit(());
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let click = ctx.link().callback(|_| SecretSwitchMessage);
        html!(
            <div class="form-check form-switch me-5">
                <input class="form-check-input" type="checkbox" onclick={click} id="toggle_cool " checked={ctx.props().secret.activated}/>
            </div>)
    }
}
