use yew::prelude::*;
use crate::view::navigation::Navigation;
use crate::util::js_object;
use crate::router::{RouterComponent};


#[derive(Clone, PartialEq, Properties, std::default::Default)]
pub struct Props {}

pub enum Msg {}

pub struct Index {
    props: Props
}

impl Component for Index {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        return Self {props: _ctx.props().clone()};
    }

    fn view(&self, _ctx:&Context<Self>) -> Html {
        html! {
            <>
                <div class="uk-width-auto uk-height-viewport uk-overflow-auto">
                    <div class="uk-height-viewport uk-flex uk-flex-center uk-flex-middle uk-background-cover uk-light"
                         data-src="https://cdn.jsdelivr.net/gh/88250/solo@f382dd9/src/main/resources/skins/Bubble/images/header-bg.jpg" uk-img="">
                            <p>{"Welcome to 《Deishy Blog》"}</p>
                    </div>
                </div>
            </>
        }
    }
}
