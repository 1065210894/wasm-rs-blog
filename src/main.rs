mod view;
mod util;
mod router;

fn main() {
    yew::Renderer::<router::Main>::new().render();
}