use yew::{function_component, html, Html};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/blog/:id")]
    Index{id: String},
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
pub fn secure() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Route::Home));
    html! {
        <div>
            { "Secure" }
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Route::Secure));
    html! {
        <div>
            { "HOME" }
            <button {onclick}>{ "Go SECURE" }</button>
        </div>
    }
}

#[function_component(Index)]
pub fn index() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Route::Secure));
    html! {
        <div>
            {"Home"}
            <button {onclick}>{ "Go SECURE" }</button>
        </div>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home =>
        html! {
            <Home />
        },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { "404" },
        Route::Index {id } => html! {
            {format!("You are looking at Post {}", id)}
        }
    }
}


#[derive(Clone, PartialEq, Properties, std::default::Default)]
pub struct Props {}

pub enum Msg {}

pub struct RouterComponent {
    props: Props
}

impl Component for RouterComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        return Self {props: _ctx.props().clone()};
    }

    fn view(&self, _ctx:&Context<Self>) -> Html {
        html! {
             <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        }
    }
}