use yew::prelude::*;
use wasm_bindgen::prelude::*;
use crate::view::navigation_content::NavigationContent;

#[derive(Clone, PartialEq, Properties, std::default::Default)]
pub struct Props {
    pub is_open: bool,
    pub navigation_style: String,
    pub navigation_width: String,
}

pub enum Msg {
    UpdateSwitch
}

pub struct Navigation {
    link: ComponentLink<Self>,
    props: Props,
}

impl Navigation {
    fn update_is_open(&mut self) {
        let is_open = self.props.is_open;
        self.props.is_open = !is_open;
        match self.props.is_open {
            true => {
                self.props.navigation_style = "left:0;".to_string();
            }
            false => {
                // let max = self.props.navigation_width.parse::<f64>().unwrap();
                // let second: f64 = 1 as f64;
                // let frames: f64 = 60 as f64;
                // let step = max / frames;
                // let wait_time = (second / frames);
                // let mut cur_width = 0 as f64;
                // for _ in 0..60 {
                //     let now  = js_sys::Date::new_0();
                //     let end_time = now.get_time() + wait_time;
                //     cur_width += step;
                //     self.props.navigation_style = "left:-".to_string() + cur_width.to_string().as_str() + "px;";
                // }
            }
        }
    }
}

impl Component for Navigation {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        return Self { link, props };
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        match _msg {
            Msg::UpdateSwitch => {
                self.update_is_open();
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let navigation_style = self.props.navigation_style.as_str();
        html! {
            <div id="navigation" style={navigation_style}>
                <span class="navigation-content-span" >
                    <NavigationContent />
                </span>
                <span class="navigation-is-open-span" >
                    <button class="navigation-is-open-btn" onclick={self.link.callback(| _| Msg::UpdateSwitch)} >
                            {if self.props.is_open {
                                "关闭"
                            } else {
                                "开启"
                            }}
                    </button>
                </span>
            </div>
        }
    }
}