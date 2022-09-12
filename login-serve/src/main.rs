use std::env;

use pretty_env_logger::init_custom_env;
use warp::Filter;

#[cfg(debug_assertions)]
const fn ipv4() -> ([u8;4], u16) {
    ([127, 0, 0, 1], 4040)
}

#[cfg(not(debug_assertions))]
const fn ipv4() -> ([u8;4], u16) {
    ([0, 0, 0, 0], 80)
}

#[tokio::main]
async fn main() {
    env::set_var("LOGIN_SERVE_LOG", "DEBUG");
    init_custom_env("LOGIN_SERVE_LOG");

    // GET /login_in/:username/:password/:timestamp
    let login_in = warp::path!("login" / "request" / "login_in" / String / String / String).map(login_in);

    let login_up = warp::path!("login" / "request" / "login_up" / String / String / String).map(login_up);

    let login = login_in.or(login_up).with(warp::log("LOGIN"));

    warp::serve(login).run(ipv4()).await;
}

fn login_in(username: String, password: String, timestamp: String) -> String {
    format!("\"success, {}, {}, {}\"", username, password, timestamp)
}

fn login_up(username: String, password: String, timestamp: String) -> String {
    format!("\"success, {}, {}, {}\"", username, password, timestamp)
}
