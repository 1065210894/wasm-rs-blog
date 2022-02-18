##项目介绍
描述：
这是关于使用yew框架编写（web项目）博客平台的样例代码。

技术栈： 

rust-wasm 

yew 框架 

yew-router 路由 

trunk 部署打包工具

## Start
cargo install trunk

启动项目: 
trunk serve --open

前提：根目录下放置index.html

css等其他静态资源需要按照trunk的官方文档提示来进行部署

打包：
trunk build 

更多trunk详细细节可以参考 https://trunkrs.dev/


##其他
等待多少毫秒后执行 demo

let end_time = js_sys::Date::new_0().get_time() + 500 as f64;
loop {
    if js_sys::Date::new_0().get_time() >= end_time {
        break;
    }
}

项目属于web项目所以需要使用到js_sys等rust包的帮助