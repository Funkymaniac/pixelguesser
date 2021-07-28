use yew::prelude::*;
use yew::web_sys::File as SysFile;

use api::{DraftRound, Round};
use pbs::Color;
use pbs::{ColumnSize, SidebarAlignment};

use crate::create::{CenterImage, SideImages, SideOptions};

pub enum Msg {
    Change(usize, DraftRound),
    Clicked(usize),
    Remove,
    Add,
}

pub struct Create {
    link: ComponentLink<Self>,
    rounds: Vec<DraftRound>,
    current: usize,
}

impl Component for Create {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, current: 0, rounds: vec![DraftRound::default()] }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Change(index, draft) => {
                self.rounds[index] = draft;
            }
            Msg::Remove => {
                self.rounds.remove(self.current);
            }
            Msg::Add => {
                self.current = self.rounds.len();
                self.rounds.push(DraftRound::default());
            }
            Msg::Clicked(index) => {
                self.current = index;
            }
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let current = self.current;

        let onchange = self.link.callback(move |draft| Msg::Change(current, draft));
        let onremove = self.link.callback(move |_| Msg::Remove);
        let onadd = self.link.callback(|_| Msg::Add);
        let onclick = self.link.callback(Msg::Clicked);

        let image = self.rounds[current].image_url.clone();
        let side_classes = "is-flex is-flex-direction-column is-justify-content-space-between";

        let images: Vec<_> = self.rounds.iter().map(|round| round.image_url.clone()).collect();
        let draft = self.rounds[self.current].clone();

        html! {
            <pbs::Columns>
                <pbs::Sidebar size=ColumnSize::Is2 alignment={SidebarAlignment::Left} extra={side_classes} overflow=false>
                    <SideImages images={images} onclick={onclick} current=self.current/>
                    <div>
                        <hr/>
                        <pbs::Buttons extra="mt-auto px-4 py-2">
                            <pbs::Button icon="fas fa-trash" fullwidth=true color={Color::Danger} light=true text="remove round" onclick={onremove}/>
                            <pbs::Button icon="fas fa-plus" fullwidth=true color={Color::Success} light=true text="add round" onclick={onadd}/>
                        </pbs::Buttons>
                    </div>
                </pbs::Sidebar>

                <pbs::Column size={ColumnSize::Is8}>
                    <CenterImage image={image} />
                </pbs::Column>

                <pbs::Sidebar size={ColumnSize::Is2} alignment={SidebarAlignment::Right} extra={format!("p-6 {}", side_classes)}>
                    <SideOptions draft={draft} onchange={onchange}/>
                </pbs::Sidebar>
            </pbs::Columns>
        }
    }
}
