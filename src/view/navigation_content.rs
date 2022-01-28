use yew::prelude::*;
use yew::html;
use crate::dto::html_dto::{HtmlByTree};

#[derive(Clone, PartialEq, Properties, std::default::Default)]
pub struct Props {}

pub enum Msg {}

pub struct NavigationContent {
    link: ComponentLink<Self>,
    props: Props,
    navigation_content: Vec<HtmlByTree>,
}

/// 获取一级目录骨架（闭包函数）
/// ```
fn get_html_tree() -> fn(&Vec<HtmlByTree>) -> Html {
    //主要目录树
    move |html_tree_vec: &Vec<HtmlByTree>| -> Html {
        html_tree_vec.into_iter().map(|content| {
            let child_li_a = get_child_li_a();
            let content_str = content.content.as_str();
            let child = &content.child;
            match content.html_type.as_str() {
                "ul li a" => {
                    html! {
                        <ul>
                            <li>
                                <a>
                                    {content_str}
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
                                    {content_str}
                                </a>
                            </li>
                            <ul>
                                {
                                    child_li_a(child)
                                }
                            </ul>
                        </ul>
                    }
                }
                _ => { html! {} }
            }
        }).collect::<Html>()
    }
}

/// 获取便利子目录的闭包函数
/// ```
fn get_child_li_a() -> fn(&Vec<HtmlByTree>) -> Html {
    //便利子目录
    move |child: &Vec<HtmlByTree>| -> Html {
        child.into_iter().map(|content| {
            let content_str = content.content.as_str();
            match content.html_type.as_str() {
                "li a" => {
                    html! {
                            <li>
                                <a>
                                    {content_str}
                                </a>
                            </li>
                        }
                }
                _ => { html! {} }
            }
        }).collect::<Html>()
    }
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
        let html_tree_vec = &self.navigation_content;
        let navigation_content = get_html_tree();
        html! {
            <>
                {
                   navigation_content(html_tree_vec)
                }
            </>
        }
    }
}