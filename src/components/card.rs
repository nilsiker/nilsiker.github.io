use yew::{html, Html};

#[derive(Clone, PartialEq)]
pub enum Card {
    Custom {
        header: Option<String>,
        image: Option<String>,
        content: Html,
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
                            <img class="outline" src="public/ferris-builder.png" width="200px" />
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
                            <img class="outline" src="public/404.png" width="200px" />
                            <h1 class="bit text-warning" style="text-align: center">{"PAGE NOT FOUND"}</h1>
                            <h2 class="bit">{"No dice, that page just ain't to be found!"}</h2>
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

                    if let Some(image) = image {<img src={image} class="card-img-top" alt="card-img-top" />}
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
