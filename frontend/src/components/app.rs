use crate::{
    components::{Gif, Ui},
    responses::gifs_response::GifsResponse,
};
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
};

#[derive(Debug)]
pub struct App {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    gifs: Vec<String>,
    gif_index: usize,
}
pub enum Msg {
    SetGifIndex(usize),
    ReceiveGifsResponse(Result<GifsResponse, anyhow::Error>),
}

impl App {
    fn fetch_gifs(&mut self) {
        let request_uri = format!(
            "https://api.giphy.com/v1/gifs/trending?api_key=q1dqaEXAGkrdq4fbGwL1pUfmM2pVWjEO&limit=10&offset={}&rating=g",
            self.gif_index,
        );
        let request = Request::get(request_uri)
            .body(Nothing)
            .expect("Could not build gifs request");
        let callback = self.link.callback(
            |response: Response<Json<Result<GifsResponse, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::ReceiveGifsResponse(data)
            },
        );
        let task = FetchService::fetch(request, callback).expect("failed to start gifs request");
        self.fetch_task = Some(task);
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_task: None,
            gifs: vec![],
            gif_index: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetGifIndex(index) => {
                self.gif_index = index;
                if self.gif_index == self.gifs.len() - 1 {
                    self.fetch_gifs();
                }
            }
            Msg::ReceiveGifsResponse(response) => {
                let gifs_response = response.expect("failed to get gifs");
                self.gifs.append(
                    &mut gifs_response
                        .data
                        .iter()
                        .map(|gif| gif.images.original.url.clone())
                        .collect(),
                );
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.fetch_gifs();
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Gif gif_index=self.gif_index gifs=self.gifs.clone() />
                <Ui gif_index=self.gif_index gifs_len=self.gifs.len() on_set_gif_index=self.link.callback(Msg::SetGifIndex) />
            </>
        }
    }
}
