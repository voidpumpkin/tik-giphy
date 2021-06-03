use yew::prelude::*;

#[derive(Debug)]
pub struct ProfileIcon;

impl Component for ProfileIcon {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 44 44"
                aria-hidden="true"
                class="icon"
                focusable="false"
            >
                <path fill-rule="evenodd" clip-rule="evenodd" d="M40 46.8125V42.4375C40 37.605 36.0825 33.6875 31.25 33.6875H13.75C8.91751 33.6875 5 37.605 5 42.4375V46.8125" />
                <path d="M40 46.8125V42.4375C40 37.605 36.0825 33.6875 31.25 33.6875H13.75C8.91751 33.6875 5 37.605 5 42.4375V46.8125" stroke-width="2.26303" stroke-linecap="round" stroke-linejoin="round"/>
                <path fill-rule="evenodd" clip-rule="evenodd" d="M22.5 24.9375C27.3325 24.9375 31.25 21.02 31.25 16.1875C31.25 11.355 27.3325 7.4375 22.5 7.4375C17.6675 7.4375 13.75 11.355 13.75 16.1875C13.75 21.02 17.6675 24.9375 22.5 24.9375Z" stroke-width="2.26303" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
        }
    }
}
