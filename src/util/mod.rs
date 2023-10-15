pub mod web_sys_utils {
    use web_sys::{Document, Storage, window, Window};

    pub fn get_window() -> Window {
        window().unwrap()
    }

    pub fn get_storage() -> Storage {
        get_window().local_storage().unwrap().unwrap()
    }

    pub fn get_document() -> Document {
        get_window().document().unwrap()
    }
}