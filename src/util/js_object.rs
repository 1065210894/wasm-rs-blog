use web_sys::{Storage, Window};

/// js中的Window对象
pub fn get_js_window() -> Window {
    web_sys::window().unwrap()
}

/// js中的local_storage对象用于存储浏览器本地缓存
pub fn get_js_storage() -> Storage {
    get_js_window().local_storage().unwrap().unwrap()
}
