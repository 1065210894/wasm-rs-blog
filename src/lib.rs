#![feature(extern_types)]

mod view;
mod dto;
mod util;

use yew::prelude::*;
use view::index::Index;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Index>::new().mount_to_body();
}