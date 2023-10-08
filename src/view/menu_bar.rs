use yew::prelude::*;

#[derive(Clone, PartialEq, Properties, std::default::Default)]
pub struct Props {}

pub enum Msg {}

pub struct MenuBar {}

impl Component for MenuBar {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        return Self {};
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="menu_bar">
                
            </div>
        }
    }
}
