use cobul::props::{Color, ColumnSize, SidebarAlignment};
use cobul::{Button, Buttons, Column, Icon, Icons, Sidebar};
use yew::prelude::*;

use api::DraftRound;

use crate::round_form::{RoundForm, RoundInfo};
use crate::round_preview::RoundPreview;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub draft: DraftRound,
    pub complete: bool,
    pub onback: Callback<()>,
    pub ondone: Callback<()>,
    pub onedit: Callback<DraftRound>,
}

#[function_component(RoundEdit)]
pub fn round_edit(props: &Props) -> Html {
    let Props { draft, onback, ondone, onedit, complete } = props.clone();

    let center = {
        let clone = draft.clone();
        let onupload = onedit.reform(move |img| DraftRound { image: Some(img), ..clone.clone() });

        html! { <RoundPreview image={draft.image.clone()} {onupload}/>}
    };

    let right = {
        let clone = draft.clone();
        let edit = move |info| {
            let RoundInfo { answer, points, guesses } = info;
            DraftRound { answer, points, guesses, ..clone.clone() }
        };

        let clone = draft.clone();
        let onremove = onedit.reform(move |_| DraftRound { image: None, ..clone.clone() });

        let info: RoundInfo = draft.into();
        let footer = html! {
            <Buttons class="mt-auto px-4 py-2">
                <Button fullwidth=true color={Color::Info} onclick={ondone} disabled={!complete}>
                    <span> {"Overview"} </span>
                </Button>
                <Button fullwidth=true color={Color::Info} light=true onclick={onback}>
                    <span> {"Back"} </span>
                </Button>
            </Buttons>
        };
        html! {
            <Sidebar size={ColumnSize::Is3} alignment={SidebarAlignment::Right} class="p-0" overflow=false footer={footer}>
                <RoundForm {info} onchange={onedit.reform(edit)} {onremove}/>
            </Sidebar>
        }
    };

    html! {
        <>
            <Column size={ColumnSize::Is7}> {center} </Column>
            {right}
        </>
    }
}
