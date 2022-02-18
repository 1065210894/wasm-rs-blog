use yew::prelude::*;
use crate::view::navigation_content::NavigationContent;

#[derive(Clone, PartialEq, Properties, std::default::Default)]
pub struct Props {
    pub is_open: bool,
}

pub enum Msg {
    UpdateSwitch
}

static mut NAVIGATION_STYLE: String = String::new();

pub struct Navigation {
    props: Props,
}

impl Navigation {
    unsafe fn update_is_open(&mut self) {
        let is_open = self.props.is_open;
        self.props.is_open = !is_open;
        let mut style = "";
        match self.props.is_open {
            true => {
                style = "animation-name: open;";
            }
            false => {
                style = "animation-name: close;";
            }
        }
        NAVIGATION_STYLE = style.to_string();
    }
}

impl Component for Navigation {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {props: _ctx.props().clone()}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateSwitch => unsafe {
                self.update_is_open();
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let navigation_style = unsafe {NAVIGATION_STYLE.as_str()};
        let is_open_click = _ctx.link().callback(|_| Msg::UpdateSwitch);
        html! {
            <div id="navigation" style={navigation_style}>
                <span class="navigation-content-span" >
                    <NavigationContent />
                </span>
                <span class="navigation-is-open-span" >
                    <button class="navigation-is-open-btn" onclick={is_open_click} >
                        {
                            if self.props.is_open {
                                "关闭"
                            } else {
                                "开启"
                            }
                        }
                    </button>
                </span>
            </div>
        }
    }
}