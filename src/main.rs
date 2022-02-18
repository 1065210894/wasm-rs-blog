#![feature(extern_types)]

mod view;
mod dto;
mod util;
mod router;
mod api;

use yew::prelude::*;

fn main() {
    yew::start_app::<view::Index>();
}