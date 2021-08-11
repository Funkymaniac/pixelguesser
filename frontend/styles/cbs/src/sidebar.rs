use yew::prelude::*;
use yew::utils::NeqAssign;

use pbs::ColumnSize;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum SidebarAlignment {
    Right,
    Left,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SidebarProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,

    #[prop_or(SidebarAlignment::Right)]
    pub alignment: SidebarAlignment,

    #[prop_or(ColumnSize::Is2)]
    pub size: ColumnSize,

    #[prop_or(true)]
    pub overflow: bool,
}

pub struct Sidebar {
    props: SidebarProps,
}

impl Component for Sidebar {
    type Message = ();
    type Properties = SidebarProps;

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
        let classes = classes!("column", &self.props.extra, self.props.size.to_string());

        let shadow = match self.props.alignment {
            SidebarAlignment::Right => "-10px 0px 10px 1px #eeeeee",
            SidebarAlignment::Left => "10px 0px 10px 1px #eeeeee",
        };

        let overflow = self.props.overflow.then(|| "overflow-y:auto").unwrap_or_default();
        let style = format!("box-shadow:{};height:100vh;{}", shadow, overflow);

        html! {
            <div class={classes} style={style}>
                { for self.props.children.iter() }
            </div>
        }
    }
}
