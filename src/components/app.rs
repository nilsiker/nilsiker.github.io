use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::{pages::about::About, switch, Route};

use super::{card::Card, navbar::*};
use yewstrap::container::*;

#[derive(PartialEq, Clone)]
pub struct SecretPackage {
    pub activated: bool,
    pub toggler: Callback<()>,
}

pub enum AppMessage {
    ToggleSecret,
}
pub struct App {
    secret: bool,
}
impl Component for App {
    type Message = AppMessage;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { secret: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        self.secret = !self.secret;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let secret = SecretPackage {
            activated: self.secret,
            toggler: ctx.link().callback(|_| AppMessage::ToggleSecret),
        };

        weblog::console_log!(self.secret.to_string());
        let hidden = self.secret;
        html!(
            <>
                <BrowserRouter>
                <Navbar secret={secret}/>
                <div id="terrain" class={if hidden {"show secret"} else {"secret"}}/>
                if !hidden {<Container fluid=false>
                    <div class="row">
                        <div  class="col mt-2">
                            <Switch<Route> render={Switch::render(switch)} />
                        </div>
                    </div>
                </Container>}

                </BrowserRouter>
            </>
        )
    }
}
