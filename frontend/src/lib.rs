#![recursion_limit = "1024"]

mod components;
mod responses;
mod utils;

use components::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::App::<App>::new().mount_to_body();
}
