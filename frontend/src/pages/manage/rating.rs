use yew::prelude::*;

use cobul::props::HeroSize;
use cobul::*;

use crate::graphql::Quiz;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub quiz: Quiz,
}

#[function_component(Rating)]
pub fn rating(props: &Props) -> Html {
    let header = html! {
        <Subtitle> {props.quiz.name.clone()} </Subtitle>
    };

    html! {
        <Hero size={HeroSize::Medium} header={header}>
            <Container extra="has-text-centered">
                <Title> {"give rating"} </Title>
                <Subtitle> {"TODO"} </Subtitle>
            </Container>
        </Hero>
    }
}
