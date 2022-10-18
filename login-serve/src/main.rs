use std::env;

use pretty_env_logger::init_custom_env;
use warp::Filter;

mod db;
mod error;
mod login_in;
mod login_up;
mod static_setting;

#[tokio::main]
async fn main() {
    env::set_var("LOGIN_SERVE_LOG", "DEBUG");
    init_custom_env("LOGIN_SERVE_LOG");

    // POST /.../:username/:password/:timestamp
    let login_in = warp::path!("login" / "request" / "login_in" / String / String / f64)
        .map(login_in::login_in);

    let login_up = warp::path!("login" / "request" / "login_up" / String / String / f64)
        .map(login_up::login_up);

    let login = login_in.or(login_up).with(warp::log("LOGIN"));

    warp::serve(login).run(static_setting::IPV4).await;
}
