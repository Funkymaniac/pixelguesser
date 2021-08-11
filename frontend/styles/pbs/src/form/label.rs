use yew::prelude::*;
use yew::utils::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LabelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,
}

pub struct Label {
    props: LabelProps,
}

impl Component for Label {
    type Message = ();
    type Properties = LabelProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let classes = classes!("label", &self.props.extra);
        html! {
            <div class={classes}>
                { for self.props.children.iter() }
            </div>
        }
    }
}
