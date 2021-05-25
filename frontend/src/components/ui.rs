use yew::prelude::*;

use crate::components::{next_button::NextButton, prev_button::PrevButton};

#[derive(Debug)]
pub struct Ui {
    props: Props,
    link: ComponentLink<Self>,
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
}

impl Component for Ui {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NextGif => {
                self.props.on_set_gif_index.emit(self.props.gif_index + 1);
            }
            Msg::PrevGif => {
                self.props.on_set_gif_index.emit(self.props.gif_index - 1);
            }
        }
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
                <div class="buttons">
                    <PrevButton gif_index=self.props.gif_index onclick=self.link.callback(|_| Msg::PrevGif) />
                    <NextButton gif_index=self.props.gif_index gifs_len=self.props.gifs_len onclick=self.link.callback(|_| Msg::NextGif) />
                </div>
                <div class="info">
                    <h1>{"Giphy/Trending"}</h1>
                </div>
            </div>
        }
    }
}
