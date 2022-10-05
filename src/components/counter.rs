use yew::prelude::*;
use yewstrap::button::*;

pub enum Msg {
    AddOne,
    SubtractOne,
}

pub struct Counter {
    value: i64,
}

#[derive(Properties, PartialEq, Eq)]
pub struct CounterProps {
    pub title: &'static str,
}

impl Component for Counter {
    type Message = Msg;
    type Properties = CounterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::SubtractOne => {
                self.value -= 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let add = link.callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::AddOne
        });
        let sub = link.callback(|e: MouseEvent| {
            e.prevent_default();
            Msg::SubtractOne
        });
        html! {
            <div>
                <h1>{ctx.props().title}</h1>
                <h2>
                    <Button
                        block=false
                        style={Style::Secondary}
                        onclick={sub}>{ "-" }
                    </Button>
                    <span class="mx-2"> { format!("{:02}", self.value)}</span>
                    <Button
                        block=false
                        style={Style::Primary}
                        onclick={add}>{ "+" }
                    </Button>
                </h2>
            </div>
        }
    }
}
