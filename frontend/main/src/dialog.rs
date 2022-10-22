use api::Mode;
use cobul::*;
use yew::*;

#[function_component(PlayDialog)]
pub fn play_dialog() -> Html {
    let model = use_model(|| Mode::Couch);
    let footer = html! {
        <Buttons>
            <Button> {"Cancel"} </Button>
            <Button> {"Save"} </Button>
        </Buttons>
    };

    html! {
        <ModalCard title="Play" {footer}>
            <simple::Tabs<Mode> {model} />
            <p> {"This is a dialog"} </p>
        </ModalCard>
    }
}
