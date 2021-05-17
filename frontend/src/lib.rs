#![recursion_limit = "1024"]

mod components;
mod responses;

use components::app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::App::<App>::new().mount_to_body();
}
