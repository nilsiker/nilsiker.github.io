mod components;
mod pages;

use pages::about::About;
use components::{card::Card, navbar::Navbar};
use yew::prelude::*;
use yew_router::prelude::*;
use yewstrap::container::*;

#[derive(Clone, Routable, PartialEq, Debug)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
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
        Route::Home => html!(),
        Route::About => html!(<About />),
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
                <div class="row">
                    <div class="col">
                        <Switch<Route> render={Switch::render(switch)} />
                    </div>
                </div>
            </Container>
            </BrowserRouter>
        </>
    )
}

fn main() {
    yew::start_app::<App>();
}
