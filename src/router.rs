use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::view::index::Index;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Index /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}