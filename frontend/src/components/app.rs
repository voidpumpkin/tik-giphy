use crate::components::{Gif, Ui};
use serde_json::Value;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        ConsoleService, FetchService,
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
    ReceiveGifsResponse(Result<Value, anyhow::Error>),
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
        let callback =
            self.link
                .callback(|response: Response<Json<Result<Value, anyhow::Error>>>| {
                    let Json(data) = response.into_body();
                    Msg::ReceiveGifsResponse(data)
                });
        let task = FetchService::fetch(request, callback).expect("failed to start gifs request");
        self.fetch_task = Some(task);
    }

    fn unpack_gifs(gifs_response: Value) -> Option<Vec<String>> {
        let gifs: Vec<String> = match gifs_response["data"].as_array() {
            Some(val) => val,
            None => {
                ConsoleService::info("No gifs were returned");
                return None;
            }
        }
        .iter()
        .filter_map(|gif| gif["images"]["original"]["url"].as_str())
        .map(|gif| gif.to_string())
        .collect();

        Some(gifs)
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
                if let Some(mut gifs) = Self::unpack_gifs(gifs_response) {
                    self.gifs.append(&mut gifs);
                }
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
