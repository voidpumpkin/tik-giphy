use yew::prelude::*;
use yewtil::NeqAssign;

use crate::components::profile_icon::ProfileIcon;

#[derive(Debug)]
pub struct ProfileButton {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    pub on_click: Callback<()>,
}

pub enum Msg {
    Click,
}

impl Component for ProfileButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.props.on_click.emit(());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <a class="button profileButton" aria-label="Profile Button" onclick=self.link.callback(|_| Msg::Click)>
                <ProfileIcon />
            </a>
        }
    }
}
