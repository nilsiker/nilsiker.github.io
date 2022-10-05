use yew::{html, Component};
use yew_feather::github::Github;
use yew_router::prelude::*;

use crate::Route;

pub struct Navbar;
impl Component for Navbar {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html!(
        <nav class=" bit navbar navbar-expand-md navbar-dark">
            <div class="container">
                <a class="navbar-brand" href="/">{"NILSIKER"}</a>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbar" aria-controls="navbar" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbar">
                    <ul class="nav navbar-nav me-0 ms-auto ">
                        <li class="nav-item">
                            <Link<Route> classes="nav-link" to={Route::Home}>{"ABOUT"}</Link<Route>>
                        </li>
                        <li class="nav-item">
                            <Link<Route> classes="nav-link" to={Route::Projects}>{"PROJECTS"}</Link<Route>>
                        </li>
                        <li class="nav-item">
                            <Link<Route> classes="nav-link" to={Route::Blog}>{"BLOG"}</Link<Route>>
                       </li>
                        <li class="nav-item dropdown v-middle">
                           <a href="https://github.com/nilsiker"><Github color="white" /></a>
                           <a href="https://twitter.com/nilsiker"><yew_feather::twitter::Twitter color="white" /></a>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
            )
    }
}
