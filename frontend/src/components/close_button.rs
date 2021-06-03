use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Debug)]
pub struct CloseButton {
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

impl Component for CloseButton {
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
            <a class="button closeButton" aria-label="Close Button" onclick=self.link.callback(|_| Msg::Click)>
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 12 12"
                    aria-hidden="true"
                    class="icon"
                    focusable="false"
                >
                    <path d="M10.192 0.34375L5.94897 4.58575L1.70697 0.34375L0.292969 1.75775L4.53497 5.99975L0.292969 10.2418L1.70697 11.6558L5.94897 7.41375L10.192 11.6558L11.606 10.2418L7.36397 5.99975L11.606 1.75775L10.192 0.34375Z" />
                </svg>
            </a>
        }
    }
}
