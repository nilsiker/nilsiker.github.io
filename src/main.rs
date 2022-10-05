mod components;

use components::{counter::*, navbar::Navbar};
use yew::prelude::*;
use yew_router::prelude::*;
use yewstrap::container::*;

#[derive(Clone, Routable, PartialEq, Debug)]
enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/blog")]
    Blog,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html!(
            <Counter title="Counter" />
        ),
        Route::Projects => Card::UnderConstruction.into(),
        Route::Blog => Card::UnderConstruction.into(),
        Route::NotFound => Card::NotFound.into(),
    }
}

#[function_component(App)]
fn app() -> Html {
    html!(
        <>
            <BrowserRouter>
            <Navbar />
            <Container fluid=false>
                <Switch<Route> render={Switch::render(switch)} />
            </Container>
            </BrowserRouter>
        </>
    )
}

fn main() {
    yew::start_app::<App>();
}

#[derive(Clone, PartialEq)]
enum Card {
    UnderConstruction,
    NotFound,
}

impl From<Card> for Html {
    fn from(card: Card) -> Self {
        match card {
            Card::UnderConstruction => {
                html!( <div class="card bg-dark mt-5 mx-auto" style="text-align:center; max-width: 35rem;">
                <div class="mt-3 card-body">
                    <img class="outline" src="ferris-builder.png" width="200px" />
                    <h1 class="bit text-warning" style="text-align: center">{"UNDER CONSTRUCTION"}</h1>
                    <h2 class="bit">{"Stick around, there might be some delicious content here one day."}</h2>
                </div>
            </div>)
            }
            Card::NotFound => {
                html!( <div class="card border-warning bg-dark mt-5 mx-auto" style="text-align:center; width: 25rem;">
                <div class="mt-5 card-body">
                    <img class="outline" src="404.png" width="200px" />
                    <h1 class="bit text-warning" style="text-align: center">{"PAGE NOT FOUND"}</h1>
                    <h2 class="bit">{"No dice, that page just ain't to be found!"}</h2>
                </div>
            </div>)
            }
        }
    }
}
