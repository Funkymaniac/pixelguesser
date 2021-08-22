use yew::prelude::*;

use pbs::prelude::*;

use graphql::Round;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub index: usize,
    pub rounds: usize,
    pub round: Round,
}

#[function_component(RoundInfo)]
pub fn round_info(props: &Props) -> Html {
    let header = html! {
        <Subtitle> {"round "} {props.index} {" / "} {props.rounds} </Subtitle>
    };

    let body = html! {
        <Container extra="has-text-centered">
            <Title> {props.round.answer.clone()} </Title>
            <Subtitle> {props.round.points} {" points"} </Subtitle>
        </Container>
    };

    html! {
        <Hero size={HeroSize::Medium} header={header} body={body}/>
    }
}
