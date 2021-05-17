use yew::prelude::*;

pub struct Gif {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub gif_index: usize,
    pub gifs: Vec<String>,
}

pub enum Msg {}

impl Component for Gif {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
        html! {
            <div class="gifContainer">
                {
                    if self.props.gifs.len() == 0 {
                        html! {  <img src="https://media.giphy.com/media/McUBKCpESJD0F7eqzT/giphy.gif" class="gif" /> }
                    } else {
                        html! {  <img src=self.props.gifs[self.props.gif_index].clone() class="gif"/> }
                    }
                }
            </div>
        }
    }
}
