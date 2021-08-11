#![allow(clippy::redundant_closure_call)]

use yew::prelude::*;
use yew::utils::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ContentProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// A single component to wrap WYSIWYG generated content, where only HTML tags are available.
///
/// [https://bulma.io/documentation/elements/content/](https://bulma.io/documentation/elements/content/)
pub struct Content {
    props: ContentProps,
}

impl Component for Content {
    type Message = ();
    type Properties = ContentProps;

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
        let classes = classes!("content", &self.props.extra);
        html! {
            <@{self.props.tag.clone()} class={classes}>
                { for self.props.children.iter() }
            </@>
        }
    }
}
