use yew::prelude::*;
use yew_feather::{aperture::Aperture, github::Github};

use crate::components::card::{Card, CardIcon};

#[derive(Properties, PartialEq)]
pub struct ContributionsPageProps {
    pub contributions: Vec<Card>,
}

#[function_component(Contributions)]
pub fn projects(props: &ContributionsPageProps) -> Html {
    
    html!(
    <>
        <div class="mt-4 p-5 bg-dark bg-opacity-75 rounded">
            <h1 class="display-4 bit">{"Contributions"}</h1>
            <p class="lead">{"Below you'll find my open-source contributions."}</p>
            <hr class="m-0 p-0" />
        </div>
        <div class="row">
            {for props.contributions.iter().map(|project| {
                html!(<div class="col-xl-4 col-md-6 col-sm-12 my-2">
                    {Html::from(project.clone())}
                </div>
                )
            })}
        </div>
    </>
    )
}

pub fn load_contributions() -> Vec<Card> {
    vec![Card::Flip {
        header: "foundry-burningwheel".into(),
        image: html!(<CardIcon icon={html!(<Aperture color="darkorange" size="100%" />)} bg="darkred"/>),
        front: html!(
            <div class="">
                <p>{"Unofficial community Foundry VTT system for The Burning Wheel RPG."}</p>
            </div>
        ),
        back: html!(
            <>
            <p>{"Unofficial community Foundry VTT system for The Burning Wheel RPG."}</p>
            <p>{"Provides character sheet support, dice rolling, and a number of automation features for The Burning Wheel. Based on the Burning Wheel Gold Revised rules available in the burning wheel store."}</p>
            <p class="fw-bold">{"Author: "}<a href="https://github.com/StasTserk">{"Stas Tserkovny"}</a></p>

            <hr />
            <p class="fw-bold">
            <a class="btn btn-light mx-2 text-dark" href="https://github.com/StasTserk/foundry-burningwheel"><Github  /></a>
            </p>
            
            </>
        ),
    }]
}