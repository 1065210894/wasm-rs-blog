mod view;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use view::index::Index;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Index>::new().mount_to_body();
}