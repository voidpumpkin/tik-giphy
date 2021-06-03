use crate::components::{profile_modal::ProfileModal, NextButton, PrevButton, ProfileButton};
use yew::prelude::*;

#[derive(Debug)]
pub struct Ui {
    props: Props,
    link: ComponentLink<Self>,
    is_profile_open: bool,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    pub gif_index: usize,
    pub gifs_len: usize,
    pub on_set_gif_index: Callback<usize>,
}

pub enum Msg {
    NextGif,
    PrevGif,
    SetProfileOpen(bool),
}

impl Component for Ui {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            is_profile_open: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NextGif => {
                self.props.on_set_gif_index.emit(self.props.gif_index + 1);
            }
            Msg::PrevGif => {
                self.props.on_set_gif_index.emit(self.props.gif_index - 1);
            }
            Msg::SetProfileOpen(val) => {
                self.is_profile_open = val;
            }
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="ui">
                <div class="topRow">
                    <h1>{"Giphy/Trending"}</h1>
                    <ProfileButton on_click=self.link.callback(|_| Msg::SetProfileOpen(true)) />
                    <ProfileModal is_open=self.is_profile_open on_close=self.link.callback(|_| Msg::SetProfileOpen(false))/>
                </div>
                <div class="buttons">
                    <PrevButton gif_index=self.props.gif_index onclick=self.link.callback(|_| Msg::PrevGif) />
                    <NextButton gif_index=self.props.gif_index gifs_len=self.props.gifs_len onclick=self.link.callback(|_| Msg::NextGif) />
                </div>
            </div>
        }
    }
}
