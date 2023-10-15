use yew::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast};
use crate::util::web_sys_utils;

#[derive(Clone, PartialEq, Properties, Default)]
pub struct Props {}

pub enum Msg {}

pub struct Index {
    _write_fn: Closure<dyn FnMut()>,
    interval_name: String,
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        return Self {
            _write_fn: Closure::new(|| {}),
            interval_name: String::from("write_handler"),
        };
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="index">
                <div class="header">
                    <div class="nes-container is-dark with-title">
                        <p class="title"> {"Welcome"} </p>
                        <div class="context">
                            <p id="show-text" > </p>
                            <span class="cursor">{"|"}</span>
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            // 动态填充文字
            self._write_fn = self.get_fill_text_fn(
                String::from("This is Liudosen's blog."),
                String::from("show-text"),
            );

            let handler = web_sys_utils::get_window()
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    self._write_fn.as_ref().unchecked_ref()
                    , 50,
                ).unwrap();

            web_sys_utils::get_storage()
                .set(&self.interval_name, &format!("{}", handler)).unwrap();
        }
    }
}

impl Index {
    fn get_fill_text_fn(&self, need_show_text: String, container_id: String) -> Closure<dyn FnMut()> {
        let already_show_index_key = "already_show_index";
        let interval_name = self.interval_name.clone();
        Closure::wrap(
            Box::new(move || {
                let document = web_sys_utils::get_document();
                let container = document.get_element_by_id(container_id.as_str()).unwrap();

                let already_show_text = container.inner_html();

                let storage = web_sys_utils::get_storage();
                let option = storage.get(already_show_index_key).unwrap();

                let index = match option {
                    None => {
                        storage.set(already_show_index_key, "0").unwrap();
                        0
                    }
                    Some(value) => {
                        let index = value.parse::<usize>().unwrap() + 1;
                        storage.set(already_show_index_key, &format!("{}", index)).unwrap();
                        index
                    }
                };

                if index >= need_show_text.len() {
                    if let Some(value) = storage.get(interval_name.as_str()).unwrap() {
                        web_sys_utils::get_window().clear_interval_with_handle(value.parse::<i32>().unwrap());
                        storage.remove_item(already_show_index_key).unwrap();
                        storage.remove_item(interval_name.as_str()).unwrap();
                    }
                    return;
                }

                let is_show_text_vec: Vec<char> = need_show_text.chars().collect();
                let next_chat = is_show_text_vec[index];
                container.set_inner_html(&format!("{}{}", already_show_text, next_chat));
            }) as Box<dyn FnMut()>
        )
    }
}