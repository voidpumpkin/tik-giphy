use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    pub gif_index: usize,
    pub onclick: Callback<()>,
}

pub enum Msg {
    Clicked,
}
#[derive(Debug)]
pub struct PrevButton {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for PrevButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        PrevButton { props, link }
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
        if self.props.gif_index != 0 {
            html! {
                <button class="prevButton" type="button" onclick=self.link.callback(|_| Msg::Clicked) />
            }
        } else {
            html! {}
        }
    }
}
