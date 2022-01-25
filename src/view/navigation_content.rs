use yew::prelude::*;
use yew::html;
use crate::component::child_li_a::ChildLiA;
use crate::dto::html_dto::{HtmlByTree};

#[derive(Clone, PartialEq, Properties, std::default::Default)]
pub struct Props {}

pub enum Msg {}

pub struct NavigationContent {
    link: ComponentLink<Self>,
    props: Props,
    navigation_content: Vec<HtmlByTree>,
}

impl Component for NavigationContent {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let content_json = "[{\"html_type\":\"ul li a\",\"content\":\"网站介绍\",\"href\":\"\",\"child\":[]},{\"html_type\":\"ul li a\",\"content\":\"作者履历\",\"href\":\"\",\"child\":[]},{\"html_type\":\"ul li\",\"content\":\"文章\",\"href\":\"\",\"child\":[{\"html_type\":\"li a\",\"content\":\"Golang\",\"href\":\"\",\"child\":[]},{\"html_type\":\"li a\",\"content\":\"Rust\",\"href\":\"\",\"child\":[]},{\"html_type\":\"li a\",\"content\":\"Java\",\"href\":\"\",\"child\":[]},{\"html_type\":\"li a\",\"content\":\"Windows开发\",\"href\":\"\",\"child\":[]},{\"html_type\":\"li a\",\"content\":\"区块链\",\"href\":\"\",\"child\":[]}]}]";
        let content_vec = serde_json::from_str::<Vec<HtmlByTree>>(content_json).unwrap();
        return Self { link: _link, props: _props, navigation_content: content_vec };
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let navigation_content = &self.navigation_content;
        html! {
            <>
                {
                   navigation_content.into_iter().map(|content| {
                        match content.html_type.as_str() {
                            "ul li a" => {
                                html! {
                                    <ul>
                                        <li>
                                            <a>
                                                {content.content.as_str()}
                                            </a>
                                        </li>
                                    </ul>
                                }
                            }
                            "ul li" => {
                                html! {
                                    <ul>
                                        <li>
                                            <a>
                                                {content.content.as_str()}
                                            </a>
                                        </li>
                                        <ChildLiA child_html_tree={&content.child} />
                                    </ul>
                                }
                            }
                            _ => { html! {} }
                        }
                    }).collect::<Html>()
                }
            </>
        }
    }
}