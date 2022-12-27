use yew::{html, Component, Properties};
use yew_feather::github::Github;
use yew_router::prelude::*;

use crate::Route;

use super::{app::SecretPackage, secret_switch::SecretSwitch};

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub secret: SecretPackage,
}
pub struct Navbar;
impl Component for Navbar {
    type Message = ();
    type Properties = NavbarProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let hidden = ctx.props().secret.activated;
        html!(
        <nav class={format!("bit navbar navbar-expand-lg navbar-dark {}", if hidden {"bg-dark"} else {""})}>
            <div class="container">
                <div class="row">
                    <div class="col">
                        <a class="navbar-brand" href="/">{"NILSIKER"}</a>
                        <SecretSwitch secret={ctx.props().secret.clone()} />
                    </div>
                </div>
                <button hidden={hidden} class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbar" aria-controls="#navbar" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbar">
                    <ul hidden={hidden} class="nav navbar-nav me-0 ms-auto ">
                        <li class="nav-item" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">
                            <Link<Route> classes="nav-link" to={Route::Projects}>{"PROJECTS"}</Link<Route>>
                        </li>
                        <li class="nav-item" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">
                            <Link<Route> classes="nav-link" to={Route::Contributions}>{"CONTRIBUTIONS"}</Link<Route>>
                        </li>
                        <li class="nav-item" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">
                            <Link<Route> classes="nav-link" to={Route::About}>{"ABOUT"}</Link<Route>>
                        </li>
                            <li class="nav-item" data-bs-toggle="collapse" data-bs-target=".navbar-collapse.show">
                        </li>
                        <li class="nav-item v-middle">
                            <a class="nav-link" href="https://nilsiker.github.io/blog">{"BLOG"}</a>
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
