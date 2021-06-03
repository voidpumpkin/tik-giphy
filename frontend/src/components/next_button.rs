use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    pub gif_index: usize,
    pub gifs_len: usize,
    pub onclick: Callback<()>,
}

pub enum Msg {
    Clicked,
}
#[derive(Debug)]
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
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        if self.props.gif_index < self.props.gifs_len {
            html! {
                <button class="nextButton" type="button" onclick=self.link.callback(|_| Msg::Clicked) />
            }
        } else {
            html! {}
        }
    }
}
