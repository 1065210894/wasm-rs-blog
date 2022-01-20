## Build
wasm-pack build --target web --out-name wasm --out-dir ./static/wasm
## Start
cargo install simple-http-server
rehash
simple-http-server --ip=127.0.0.1 -p=8080 --index=index.html static

## 等等多少毫秒后执行的代码
let end_time = js_sys::Date::new_0().get_time() + 500 as f64;
loop {
    if js_sys::Date::new_0().get_time() >= end_time {
        break;
    }
}

## 弹窗
#[wasm_bindgen]
extern {
fn alert(s: &str);
}