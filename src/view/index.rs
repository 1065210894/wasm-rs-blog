use yew::prelude::*;
use wasm_bindgen::prelude::*;
use crate::view::navigation::Navigation;
use crate::util::js_object;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[derive(Clone, PartialEq, Properties, std::default::Default)]
pub struct Props {
    pub height: i32,
    pub width: i32,
}

pub enum Msg {}

pub struct Index {
    _link: ComponentLink<Self>,
    props: Props,
}

impl Component for Index {
    type Message = Msg;
    type Properties = Props;

    fn create(mut props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let document:web_sys::Document = js_object::get_js_window().document().expect("should have a document on window");
        let element:web_sys::Element = document.document_element().expect("document should have a body");
        props.height = element.client_height();
        props.width = element.client_width();
        return Self { _link, props };
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let height_str = "height:";
        let width_str = "width:";
        let style_end = "px;";

        let height = self.props.height.to_string();
        let width = self.props.width.to_string();

        let style: String = height_str.to_string() + &height + style_end +
            width_str + &width + style_end;
        html! {
            <div class="index" style={style}>
                <Navigation is_open={true} navigation_style={"".to_string()} />
            </div>
        }
    }
}
