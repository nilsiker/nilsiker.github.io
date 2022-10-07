use yew::prelude::*;

use crate::components::card::Card;


#[function_component(About)]
pub fn about() -> Html {
    Card::Custom {
            header: None,
            image: Some("public/pb.png".into()),
            content: html!(
                <div>
                    <h1 class="bit mb-0" style="line-height: 2rem">{"Andreas Nilsson"}</h1>
                    <h2 class="bit mt-0 mb-4 text-muted" style="line-height: 2rem">{"Web and Software Developer"}</h2>
                    <hr/>
                    <p>{"Malmö-based millenial living with my SO and cats. Trying to grow habaneros on our roof terrace."}</p>
                    <p><b>{"Talk to me about"}</b></p>
                    <ul style="line-height: 0.5rem">
                        <p >{"🦀 All things code "}</p>
                        <p>{"🎲 Tabletop RPGs"}</p>
                        <p>{"🎵 Folk music and progressive metal"}</p>
                        <p>{"🍻 Craft beers and whiskey "}</p>
                    </ul>
                    <p>{"I'm constantly looking for new tech and tools to help grow my coding and project management skills. Preferably by building digital tools for tabletop games!"}</p>
                </div>

            ),
            }.into()
}
