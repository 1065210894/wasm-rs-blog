use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::view::{Index};

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Index,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <>
            <div class="root">
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </div>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! {
            <Index />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}