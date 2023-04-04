mod view;
mod util;

use view::index::Index;

fn main() {
    yew::Renderer::<Index>::new().render();
}