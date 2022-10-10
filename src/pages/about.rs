use yew::prelude::*;
use yew_feather::{book::Book, briefcase::Briefcase, user::User};

use crate::components::card::Card;

#[function_component(About)]
pub fn about() -> Html {
    let personal: Html = Card::Custom {
            header: None,
            image: Some("static/pb.png".into()),
            content: html!(
                <div>
                <User/>
                    <h1 class="bit mb-0" style="line-height: 2rem">{"Andreas Nilsson"}</h1>
                    <h2 class="bit mt-0 mb-4 text-muted" style="line-height: 2rem">{"Web and Software Developer"}</h2>
                    <hr/>
                    <p>{"Hi, I'm Andreas, a Malmö-based millenial living with my SO and cats. Trying to grow habaneros on our roof terrace."}</p>
                    <p><b>{"Talk to me about"}</b></p>
                    <ul style="line-height: 0.5rem">
                        <p >{"🦀 All things code "}</p>
                        <p>{"🎲 Tabletop RPGs"}</p>
                        <p>{"🎵 Folk music and progressive metal"}</p>
                        <p>{"🍻 Craft beers and whiskey "}</p>
                    </ul>
                    <p class="mb-0">{"I constantly find ways to talk about the programming language Rust, so there's a good chance most of this page will be about just that. I've warned you!"}</p>
                </div>
            ),
            }.into();

    let career: Html = Card::Custom {
            header: None,
            image: None,
            content: html!(
                <div>
                    <div>
                        <Briefcase />
                        <h2 class="bit mb-0" style="line-height: 2rem">{"System Developer"}</h2>
                        <h4 class="bit text-muted mb-1" style="line-height:1rem;">{"2022-ongoing"}</h4>
                        <h3 class="bit text-muted" style="line-height: 2rem">{"Skanska IT"}</h3>
                        <hr/>
                        <p>{"I currently work as a full-time system developer at Skanska in Malmö, where I am part of the Digital Collaboration team, focusing on SharePoint and Azure development."}</p>
                        <span class="badge bg-primary rounded-pill me-1">{"Azure"}</span>
                        <span class="badge bg-primary rounded-pill me-1">{"Microsoft 365"}</span>
                        <span class="badge bg-success rounded-pill me-1">{"SPFx"}</span>
                        <span class="badge bg-success rounded-pill me-1">{"React"}</span>
                        <span class="badge bg-danger rounded-pill me-1">{"TypeScript"}</span>
                    </div>
                    <div class="mt-5">
                    <h2 class="bit my-0" style="line-height: 2rem">{"Developer"}</h2>
                    <h4 class="bit text-muted mb-1" style="line-height:1rem;">{"2021-2022"}</h4>
                        <h3 class="bit text-muted" style="line-height: 2rem">{"Netcompany A/S"}</h3>

                        <hr/>
                        <p>{"I worked as a full-time software developer at Netcompany in Copenhagen, where I built and maintained solutions for a plethora of customers, both big and small."}</p>
                        <span class="badge bg-primary rounded-pill me-1">{"Dynamics 365"}</span>
                        <span class="badge bg-primary rounded-pill me-1">{"Salesforce"}</span>
                        <span class="badge bg-danger rounded-pill me-1">{".NET"}</span>
                        <span class="badge bg-danger rounded-pill me-1">{"JavaScript"}</span>
                    </div>
                </div>
            ),
            }.into();

    let education: Html = Card::Custom {
            header: None,
            image: None,
            content: html!(
                <div>
                <Book />
                <h2 class="bit mb-0" style="line-height: 2rem">{"Computer Science and Engineering."}</h2>
                <h4 class="bit text-muted mb-1" style="line-height:1rem;">{"2018-2021"}</h4>
                <h3 class="bit text-muted" style="line-height: 2rem">{"LTH, Faculty of Engineering"}</h3>
                    <hr/>
                    <p>{"During my "}<span>{"Bachelor of Science in Engineering, Computer Science and Engineering"}</span>{" (that's a mouthful...), I laid the foundation for my programming, software design and project management skills."}</p>
                    <p>{"For my thesis I designed, developed and evaluated an .NET OpenID Connect authentication module for EPiServer, using a self-designed test specification based on the "}<a href="https://owasp.org/www-project-web-security-testing-guide/">{"OWASP Web Security Testing Guide"}</a></p>
                </div>
            ),
            }.into();

    html!(
        <div class="row">
            <div class="col-xl-4 col-xs-12 mb-3">{personal}</div>
            <div class="col-xl-4 col-xs-12 mb-3">{career}</div>
            <div class="col-xl-4 col-xs-12 mb-3">{education}</div>
        </div>)
}
