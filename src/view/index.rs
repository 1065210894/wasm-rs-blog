use yew::prelude::*;
use crate::view::navigation::Navigation;
use crate::util::js_object;
use crate::router::{RouterComponent};


#[derive(Clone, PartialEq, Properties, std::default::Default)]
pub struct Props {}

pub enum Msg {}

pub struct Index {
    props: Props,
    height: i32,
    width: i32,
}

impl Component for Index {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        let document:web_sys::Document = js_object::get_js_window().document().expect("should have a document on window");
        let element:web_sys::Element = document.document_element().expect("document should have a body");
        let height = element.client_height();
        let width = element.client_width();
        return Self {props: _ctx.props().clone(), height, width };
    }

    fn view(&self, _ctx:&Context<Self>) -> Html {
        let style_end = "px;";
        let height = self.height.to_string();
        let width = self.width.to_string();

        let style: String = "height:".to_string() + &height + style_end +
            "width:" + &width + style_end;
        html! {
            <>
                <div class="index" style={style}>
                    <Navigation is_open={true} />
                    <div class="index_content" >
                        <RouterComponent />
                    </div>
                </div>
            </>
        }
    }
}
