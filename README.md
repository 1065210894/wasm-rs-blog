## Build
wasm-pack build --target web --out-name wasm --out-dir ./static/wasm
## Start
cargo install simple-http-server
rehash
simple-http-server --ip=127.0.0.1 -p=8080 --index=index.html static