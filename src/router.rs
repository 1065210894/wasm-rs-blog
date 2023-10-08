use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::util::js_object;

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
    let window = js_object::get_js_window();
    // 获取可视宽高
    let window_height = window.inner_height().unwrap().as_f64().unwrap();
    let window_width = window.inner_width().unwrap().as_f64().unwrap();

    // 配置动态宽度
    let mut dynamic_width = window_width.clone();

    // 窄屏
    if window_height >= window_width {
        dynamic_width = window_width * 0.98;
    } else {
        // 宽屏
        dynamic_width = window_width * 0.8;
    }

    html! {
        <>
            <div class="index">
                <div class="root nes-container is-rounded is-dark is-centered" style={format!("width:{}px;", dynamic_width)}>
                    <BrowserRouter>
                        <Switch<Route> render={switch} />
                    </BrowserRouter>
                </div>
            </div>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <p>{"这里是《Liudosen blog》"}</p>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}