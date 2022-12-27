use yew::prelude::*;
use yew_feather::{book::Book, github::Github, link::Link, shield::Shield};

use crate::components::card::{Card, CardIcon, CardImage};

#[derive(Properties, PartialEq)]
pub struct ProjectsPageProps {
    pub projects: Vec<Card>,
}

#[function_component(Projects)]
pub fn projects(props: &ProjectsPageProps) -> Html {
    html!(
        <>
            <div class="mt-4 p-5 bg-dark bg-opacity-75 rounded">
                <h1 class="display-4 bit">{"Projects"}</h1>
                <p class="lead">{"Welcome to my project portfolio!"}</p>
                <p class="lead">{"Below you'll find stuff that I have worked on, or am currently working on!"}</p>
                <hr class="m-0 p-0" />
            </div>
            <div class="row">
                {for props.projects.iter().map(|project| {
                    html!(<div class="col-xl-4 col-md-6 col-sm-12 my-2">
                        {Html::from(project.clone())}
                    </div>
                    )
                })}
            </div>
        </>
    )
}

pub fn load_projects() -> Vec<Card> {
    vec![
        Card::Flip {
            header: "Contour".into(),
            image: html!(<CardImage bg="black" src="static/contour.png" />),
            front: html!(
                <div class="">
                    {"A pixel-art horror noir detective game, powered by Rust and Bevy."}
                </div>
            ),
            back: html!(
                <>
                    <p>{"A pixel-art horror noir detective game, powered by Rust and Bevy."}</p>
                    <p class="fst-italic">{"A private investigator takes on a seemingly routine missing person case, only to find himself in a sinister mystery beyond comprehension."}</p>
                    <p>{"In active development."}</p>
                    <hr />
                    <a class="btn btn-danger" href="https://nilsiker.itch.io/contour" type="button">{"Play on itch.io"}</a>
                    <a class="btn btn-light mx-2 text-dark" href="https://github.com/nilsiker/contour"><Github /></a>
                </>
            ),
        },
        Card::Flip {
            header: "bevy_ymir".into(),
            image: html!(<CardImage fit=true bg="transparent" src="static/ymir-early-world.png" />),
            front: html!(
                <div class="">
                    {"A procedural world generator plugin for Bevy, a Rust game engine."}
                </div>
            ),
            back: html!(
                <div>
                    <p>{"A plugin for generating and streaming procedural worlds in Bevy."}</p>
                    <p>{"The ambition is to provide a customizable world generator, with support for different biomes and various methods for procedural object placement."}</p>
                    <p>{"Keep in mind that Ymir is in very early development. Expect hard-to-use APIs that break constantly!"}</p>
                    <hr/>
                    <a type="button" class="btn btn-light mx-2 text-dark" href="https://github.com/nilsiker/bevy_ymir"><Github /></a>
                </div>
            ),
        },
        Card::Flip {
            header: "nilsiker blog".into(),
            image: html!(<CardIcon icon={html!(<Book color="white" size="100%"/>)} bg="#55ff8c77" />),
            front: html!(
                <div class="">
                    {"My blog and news site powered by Zine. This is were I keep my personal rants and ramblings."}
                </div>
            ),
            back: html!(
                <div>
                    <p>{"Alongside this portfolio page, I keep a Zine site where I post about my code endeavours and occassional slice-of-life posts."}</p>
                    <p>{"The blog also serves as a devlog for my various projects."}</p>
                    <p>{"If you're looking for a more relaxed everyday-Andreas, chances are you'll find him more easily over the blog!"}</p>
                    <hr/>
                    <a type="button" class="btn btn-light mx-2 text-dark" href="https://nilsiker.github.io/blog"><Link /></a>
                    <a type="button" class="btn btn-light mx-2 text-dark" href="https://github.com/nilsiker/blog"><Github /></a>

                </div>
            ),
        },
        Card::Flip {
            header: "nilsiker.github.io".into(),
            image: html!(<CardImage fit=true bg="transparent" src="static/unsplash.jpg" />),
            front: html!(
                <div class="">
                    {"The very page you're looking at now, delivered to you with Rust and Yew."}
                </div>
            ),
            back: html!(
                <div>
                    <p>{"With the risk that this becomes a bit meta, I am also actively working on this portfolio website!"}</p>
                    <p>{"At the moment, it is a static Yew website written by me, an all right Rust developer, but definitely a rookie Yew developer."}</p>
                    <p class="fst-italic">{"If you want to hunt for a secret, remember that some underlines are just for show."}</p>
                    <hr/>
                    <a type="button" class="btn btn-light mx-2 text-dark" href="https://github.com/nilsiker/nilsiker.github.io"><Github /></a>

                </div>
            ),
        },
    ]
}
