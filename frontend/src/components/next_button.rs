use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub gif_index: usize,
    pub gifs_len: usize,
    pub onclick: Callback<()>,
}

pub enum Msg {
    Clicked,
}
pub struct NextButton {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for NextButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NextButton { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.props.onclick.emit(());
            }
        }
        false
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
        {
            if self.props.gif_index != self.props.gifs_len - 1 {
                html! {
                    <button class="nextButton" type="button" onclick=self.link.callback(|_| Msg::Clicked) />
                }
            } else {
                html! {}
            }
        }
    }
}