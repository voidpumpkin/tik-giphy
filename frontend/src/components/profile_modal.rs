use crate::{
    components::{CloseButton, ProfileIcon},
    log,
};
use yew::prelude::*;

#[derive(Debug)]
pub struct ProfileModal {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    pub is_open: bool,
    pub on_close: Callback<()>,
}

pub enum Msg {
    Close,
}

impl Component for ProfileModal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Close => {
                self.props.on_close.emit(());
            }
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        log!("{:?}", props);
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        if self.props.is_open {
            html! {
                <div class="profile">
                    <CloseButton on_click=self.link.callback(|_| Msg::Close)/>
                    <div class="profileDataRow">
                        <div class="profileIcon">
                            <ProfileIcon />
                        </div>
                        <div class="profileTextWrapper">
                            <p class="username">{"Guest"}</p>
                            <p class="signIn">{"Please sign in"}</p>
                        </div>
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    }
}
