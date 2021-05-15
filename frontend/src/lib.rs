#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use yew::{prelude::*, services::ConsoleService};

struct Model {
    link: ComponentLink<Self>,
    gifs: Vec<&'static str>,
    gifIndex: usize,
}

enum Msg {
    NextGif,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            gifs: vec![
                "https://media.giphy.com/media/1rGIUIAEaYPAUr6WFG/giphy.gif",
                "https://media.giphy.com/media/2dQ3FMaMFccpi/giphy.gif",
            ],
            gifIndex: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NextGif => self.gifIndex = self.gifIndex + 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="gifSlider" ontouchstart=self.link.callback(|_| Msg::NextGif)>
                <img src=self.gifs[self.gifIndex] class="gif"/>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
