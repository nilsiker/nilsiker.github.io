mod components;
mod pages;

use components::{app::App, card::Card};
use pages::about::About;
use yew::{html, Html};
use yew_router::Routable;

fn main() {
    yew::start_app::<App>();
}


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