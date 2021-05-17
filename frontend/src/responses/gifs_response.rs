use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct OrgiginalGifImage {
    pub url: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GifImage {
    pub original: OrgiginalGifImage,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Gif {
    pub images: GifImage,
}
#[derive(Deserialize, Debug, Clone)]
pub struct GifsResponse {
    pub data: Vec<Gif>,
}
