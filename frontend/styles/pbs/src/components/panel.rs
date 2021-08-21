// #![allow(clippy::redundant_closure_call)]
//
// use yew::events::MouseEvent;
// use yew::prelude::*;
// use yew::utils::NeqAssign;
//
// #[derive(Clone, Debug, Properties, PartialEq)]
// pub struct PanelProps {
//     #[prop_or_default]
//     pub children: Children,
//     #[prop_or_default]
//     pub extra: String,
//     /// The HTML content of this panel's heading; it is automatically wrapped in a `p.panel-heading`.
//     #[prop_or_default]
//     pub heading: Html,
// }
//
// /// A composable panel, for compact controls.
// ///
// /// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
// pub struct Panel {
//     props: PanelProps,
// }
//
// impl Component for Panel {
//     type Message = ();
//     type Properties = PanelProps;
//
//     fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
//         Self { props }
//     }
//
//     fn update(&mut self, _: Self::Message) -> ShouldRender {
//         false
//     }
//
//     fn change(&mut self, props: Self::Properties) -> ShouldRender {
//         self.props.neq_assign(props)
//     }
//
//     fn view(&self) -> Html {
//         let classes = classes!("panel", &self.props.extra);
//         html! {
//             <nav class={classes}>
//                 <p class="panel-heading">{ self.props.heading.clone() }</p>
//                 { for self.props.children.iter() }
//             </nav>
//         }
//     }
// }
//
// //////////////////////////////////////////////////////////////////////////////
// //////////////////////////////////////////////////////////////////////////////
//
// #[derive(Clone, Debug, Properties, PartialEq)]
// pub struct PanelTabsProps {
//     #[prop_or_default]
//     pub children: Children,
// }
//
// /// A container for the navigation tabs of a panel.
// ///
// /// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
// pub struct PanelTabs {
//     props: PanelTabsProps,
// }
//
// impl Component for PanelTabs {
//     type Message = ();
//     type Properties = PanelTabsProps;
//
//     fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
//         Self { props }
//     }
//
//     fn update(&mut self, _: Self::Message) -> ShouldRender {
//         false
//     }
//
//     fn change(&mut self, props: Self::Properties) -> ShouldRender {
//         self.props.neq_assign(props)
//     }
//
//     fn view(&self) -> Html {
//         html! {
//             <p class="panel-tabs">
//                 {self.props.children.clone()}
//             </p>
//         }
//     }
// }
//
// //////////////////////////////////////////////////////////////////////////////
// //////////////////////////////////////////////////////////////////////////////
//
// #[derive(Clone, Debug, Properties, PartialEq)]
// pub struct Props {
//     #[prop_or_default]
//     pub children: Children,
//     /// The HTML tag to use for this component.
//     #[prop_or_else(|| "div".into())]
//     pub tag: String,
//     /// Make this element the active / highlighted element.
//     #[prop_or_default]
//     pub active: Active,
//     /// The click handler for this element.
//     #[prop_or_else(Callback::noop)]
//     pub onclick: Callback<()>,
// }
//
// /// An individual element of the panel.
// ///
// /// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
// #[function_component(Tabs)]
// pub fn tabs(props: &Props) -> Html {
//     let classes = classes!("panel-block", props.active);
//
//     html! {
//         <@{props.tag.clone()} class={classes} onclick={props.onclick.clone()}>
//             { for props.children.iter() }
//         </@>
//     }
// }
