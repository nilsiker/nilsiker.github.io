mod components;
mod pages;

use components::{app::App, card::Card};
use pages::{
    about::About,
    contributions::{load_contributions, Contributions},
    projects::{load_projects, Projects},
};
use yew::{html, Html};
use yew_router::{Routable, prelude::Redirect};

fn main() {
    yew::start_app::<App>();
}

#[derive(Clone, Routable, PartialEq, Debug)]
enum Route {
    #[at("/")]
    Index,
    #[at("/projects")]
    Projects,
    #[at("/contributions")]
    Contributions,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Index | Route::Projects => html!(<Projects projects={load_projects()} />),
        Route::Contributions => html!(<Contributions contributions={load_contributions()} />),
        Route::About => html!(<About />),
        Route::NotFound => Card::NotFound.into(),
        
    }
}
