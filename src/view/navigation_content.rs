use yew::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use yew::html;

#[derive(Clone, PartialEq, Properties, std::default::Default)]
pub struct Props {}

pub enum Msg {
    SwitchUl(i8),
}

#[derive(Serialize, Deserialize)]
pub struct Content {
    html_type: String,
    content: String,
    href: String,
    child: Vec<Content>,
}

pub struct NavigationContent {
    link: ComponentLink<Self>,
    props: Props,
    navigation_content: Vec<Content>,
}

// impl Content {
//     pub fn get_child_html(&self) -> Html {
//         html! {
//              <ul class={"navigation-ul-".to_string() + index.to_string().as_str()}>
//                 <li>
//                     {
//                         &self.child.into_iter().map(|content| {
//                             match content.html_type.as_str() {
//                                 "li a" => {
//                                     html! {
//                                         <li>
//                                             <a>
//                                                 {content.content.as_str()}
//                                             </a>
//                                         </li>
//                                     }
//                                 }
//                                 _ => { html! {} }
//                             }
//                          }).collect::<Html>()
//                     }
//                 </li>
//             </ul>
//         }
//     }
// }

impl Component for NavigationContent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let content_json = "[{\"html_type\":\"ul li a\",\"content\":\"网站介绍\",\"href\":\"\",\"child\":[]},{\"html_type\":\"ul li a\",\"content\":\"作者履历\",\"href\":\"\",\"child\":[]},{\"html_type\":\"ul li\",\"content\":\"文章\",\"href\":\"\",\"child\":[{\"html_type\":\"li a\",\"content\":\"Golang\",\"href\":\"\",\"child\":[]},{\"html_type\":\"li a\",\"content\":\"Rust\",\"href\":\"\",\"child\":[]},{\"html_type\":\"li a\",\"content\":\"Java\",\"href\":\"\",\"child\":[]},{\"html_type\":\"li a\",\"content\":\"Windows开发\",\"href\":\"\",\"child\":[]},{\"html_type\":\"li a\",\"content\":\"区块链\",\"href\":\"\",\"child\":[]}]}]";
        let content_vec = serde_json::from_str::<Vec<Content>>(content_json).unwrap();
        return Self { link, props, navigation_content: content_vec };
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
                                        {
                                            &content.child.into_iter().map(|content| {
                                                match content.html_type.as_str() {
                                                    "li a" => {
                                                        html!{
                                                            <li>
                                                                <a>
                                                                    {&content.content.as_str()}
                                                                </a>
                                                            </li>
                                                        }
                                                    }
                                                    _ => {html!{}}
                                                }
                                            }).collect::<Html>()
                                        }
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