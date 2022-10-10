use yew::{function_component, html, Html, Properties};

#[derive(Clone, PartialEq)]
pub enum Card {
    Custom {
        header: Option<String>,
        image: Option<String>,
        content: Html,
    },
    Flip {
        header: String,
        image: Html,
        front: Html,
        back: Html,
    },
    UnderConstruction,
    NotFound,
}

impl From<Card> for Html {
    fn from(card: Card) -> Self {
        match card {
            Card::UnderConstruction => {
                html!(
                    <div class="card bg-dark mx-auto" style="text-align:center; max-width: 30rem;">
                        <div class="mt-3 card-body">
                            <img class="outline" src="static/ferris-builder.png" width="200px" />
                            <h1 class="bit text-warning" style="text-align: center">{"UNDER CONSTRUCTION"}</h1>
                            <h2 class="bit">{"Stick around, there might be some delicious content here one day."}</h2>
                        </div>
                    </div>
                )
            }
            Card::NotFound => {
                html!(
                    <div class="card border-warning bg-dark mx-auto" style="text-align:center; max-width: 25rem;">
                        <div class="mt-5 card-body">
                            <img class="outline" src="static/404.png" width="200px" />
                            <h1 class="bit text-warning" style="text-align: center">{"PAGE NOT FOUND"}</h1>
                            <h2 class="bit">{"No dice, that page just ain't to be found!"}</h2>
                        </div>
                    </div>
                )
            }
            Card::Flip {
                header,
                image,
                front,
                back,
            } => {
                html!(
                    <div class="card p-2 bg-dark mx-auto flip" style="max-width: 30rem;">
                        <div class="content">
                            <div class="back">
                                <div class="card-body">
                                    <h3 class="bit card-title">{header.clone()}</h3>
                                    <hr/>
                                    {back}
                                </div>
                            </div>
                            <div class="front">
                                {image}
                                <h1 class="card-header bit">{header}</h1>
                                <div class="card-body">
                                    {front}
                                </div>
                            </div>
                        </div>
                    </div>
                )
            }
            Card::Custom {
                image,
                header,
                content,
            } => {
                html!(
                    <div class="card p-2 bg-dark mx-auto" style="max-width: 30rem;">

                    if let Some(image) = image {<img src={image} class="card-img-top bg-light" alt="card-img-top" />}
                    if let Some(header) = header {<h1 class="card-header bit">{header}</h1>}
                        <div class="card-body">
                            {content}
                        </div>
                    </div>
                )
            }
        }
    }
}

#[derive(PartialEq, Eq, Properties)]
pub struct CardImageProps {
    pub src: &'static str,
    pub bg: &'static str,
    #[prop_or_default]
    pub fit: bool,
}

#[function_component(CardImage)]
pub fn card_image(props: &CardImageProps) -> Html {
    html!(
        <div class="text-center rounded-top" style={format!("background-color: {}; {}",props.bg, if props.fit {"object-fit: cover"} else {""})}>
            <img src={props.src} class="card-img-top mx-auto" alt="card-img-top" style={format!("background-color:black; height:250px; {}", if props.fit {""} else {"width:auto"})} />
        </div>
    )
}

#[derive(PartialEq, Properties)]
pub struct CardIconProps {
    pub icon: Html,
    pub bg: &'static str,
}

#[function_component(CardIcon)]
pub fn card_icon(props: &CardIconProps) -> Html {
    html!(
        <div class="card-img-top" alt="card-img-top" style={format!("height: 250px; background-color:{}", props.bg)}>
            {props.icon.clone()}
        </div>
    )
}
