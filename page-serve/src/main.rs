use std::env;

use pretty_env_logger::init_custom_env;
use warp::Filter;

#[cfg(debug_assertions)]
const fn ipv4() -> ([u8;4], u16) {
    ([127, 0, 0, 1], 5050)
}

#[cfg(not(debug_assertions))]
const fn ipv4() -> ([u8;4], u16) {
    ([0, 0, 0, 0], 80)
}

#[tokio::main]
async fn main() {
    env::set_var("LOGIN_SERVE_LOG", "DEBUG");
    init_custom_env("LOGIN_SERVE_LOG");

    let index = warp::get().and(warp::fs::dir("./web/dist/"));

    let route_suit = warp::get().and(warp::fs::file("./web/dist/index.html"));

    let page_serve = index.or(route_suit).with(warp::log("LOGIN"));

    warp::serve(page_serve).run(ipv4()).await;
}