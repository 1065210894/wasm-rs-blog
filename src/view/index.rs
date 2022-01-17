use yew::prelude::*;

pub enum Msg {}

pub struct Index {
    link: ComponentLink<Self>,
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        return Self { link };
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="index">
                {"this deishy blog"}
            </div>
        }
    }
}
