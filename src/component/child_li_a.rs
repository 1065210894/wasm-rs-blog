use yew::prelude::*;
use yew::Properties;
use wasm_bindgen::prelude::*;
use crate::dto::html_dto::HtmlByTree;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[derive(Clone, Properties, std::default::Default)]
pub struct Props {
    pub child_html_tree: Vec<HtmlByTree>,
}

pub enum Msg {}

pub struct ChildLiA {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for ChildLiA {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        return Self { link: _link, props };
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        // let get = move || -> Html {
        //     html!{
        //         <a>
        //             {"你好"}
        //         </a>}
        // };
        let child_html_tree = &self.props.child_html_tree;
        html! {
            <>
                <ul>
                    {
                        child_html_tree.into_iter().map(|content| {
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
            </>
        }
    }
}