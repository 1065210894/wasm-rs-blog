use yew::prelude::*;
use crate::view::navigation_content::NavigationContent;

#[derive(Clone, PartialEq, Properties, std::default::Default)]
pub struct Props {
    pub is_open: bool,
    pub navigation_style: String,
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
                let style = "animation-name: open;";
                self.props.navigation_style = style.to_string();
            }
            false => {
                let style = "animation-name: close;";
                self.props.navigation_style = style.to_string();
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
        let is_open_click = self.link.callback(|_| Msg::UpdateSwitch);
        html! {
            <div id="navigation" style={navigation_style}>
                <span class="navigation-content-span" >
                    <NavigationContent />
                </span>
                <span class="navigation-is-open-span" >
                    <button class="navigation-is-open-btn" onclick={is_open_click} >
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