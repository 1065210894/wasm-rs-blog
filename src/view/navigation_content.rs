use yew::prelude::*;

#[derive(Clone, PartialEq, Properties, std::default::Default)]
pub struct Props {}

pub enum Msg {}

pub struct NavigationContent {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for NavigationContent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        return Self { link, props };
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
                <>
                    <ul>
                        <li>
                            <a>
                                {"网站介绍"}
                            </a>
                        </li>
                    </ul>
                    <ul>
                        <li>
                            <a>
                                {"作者简历"}
                            </a>
                        </li>
                    </ul>
                    <ul>
                        <li>
                            <a>
                                {"文章"}
                            </a>
                        </li>
                        <ul>
                            <li>
                                <a>
                                    {"Golang"}
                                </a>
                            </li>
                            <li>
                                <a>
                                    {"Rust"}
                                </a>
                            </li>
                            <li>
                                <a>
                                    {"Java"}
                                </a>
                            </li>
                            <li>
                                <a>
                                    {"windows开发"}
                                </a>
                            </li>
                            <li>
                                <a>
                                    {"区块链"}
                                </a>
                            </li>
                        </ul>
                    </ul>
                </>
        }
    }
}