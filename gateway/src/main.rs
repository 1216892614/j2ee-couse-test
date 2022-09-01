use std::{env, fs::File, io::Read};

use pretty_env_logger::init_custom_env;
use toml::Value;

mod listen;
mod proxy;

#[cfg(debug_assertions)]
fn get_gateway_toml_file() -> Result<File, std::io::Error> {
    File::open(format!("{}/Gateway.toml", env!("CARGO_MANIFEST_DIR")))
}

#[cfg(not(debug_assertions))]
fn get_gateway_toml_file() -> Result<File, std::io::Error> {
    File::open("./Gateway.toml")
}

fn main() {
    env::set_var("LOGIN_SERVE_LOG", "DEBUG");
    init_custom_env("LOGIN_SERVE_LOG");

    let gateway_toml = read_gateway_toml_file();
}

fn read_gateway_toml_file() -> String {
    let mut file = get_gateway_toml_file().expect("Can't find expect Gateway.toml file");
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)
        .expect("Gateway.toml file is not a text file");

    buffer
}

fn fmt_gateway_toml_to_listens_and_proxys(
    gateway_toml: String,
) -> (Vec<listen::Listen>, Vec<proxy::Proxy>) {
    let value = gateway_toml.parse::<Value>().unwrap();
    let gateway_toml = value.as_table().expect("Cannot fmt ur Gateway.toml!");

    let listens = fmt_gateway_toml_map_to_listens(gateway_toml);

    let proxys = fmt_gateway_toml_map_to_proxys(gateway_toml);

    (listens, proxys)
}

fn fmt_gateway_toml_map_to_listens(
    gateway_toml: &toml::value::Map<String, Value>,
) -> Vec<listen::Listen> {
    if let Some(listens_toml) = gateway_toml.get("listens") {
        let mut listeners = Vec::new();

        for value in listens_toml
            .as_array()
            .expect("Set listens like [[listens]].")
        {
            listeners.push(fmt_value_to_listen(value));
        }

        listeners
    } else {
        Vec::new()
    }
}

fn fmt_value_to_listen(value: &Value) -> listen::Listen {
    let listen = value
        .as_table()
        .expect("Do NOT set listeners like listens = [[...], [...], ...]");

    let enable = listen
        .get("enable")
        .unwrap_or(&Value::String(String::new()))
        .as_str()
        .unwrap_or("")
        .to_string();

    let url = listen
        .get("url")
        .expect("U have to set a url for listens")
        .as_str()
        .expect("url set for listens can only be str")
        .to_string();

    listen::Listen::from(enable, url)
}

fn fmt_gateway_toml_map_to_proxys(
    gateway_toml: &toml::value::Map<String, Value>,
) -> Vec<proxy::Proxy> {
    if let Some(proxys_toml) = gateway_toml.get("proxys") {
        let mut proxys = Vec::new();

        for value in proxys_toml.as_array().expect("Set proxys like [[proxys]].") {
            proxys.push(fmt_value_to_proxy(value));
        }

        proxys
    } else {
        Vec::new()
    }
}

fn fmt_value_to_proxy(value: &Value) -> proxy::Proxy {
    todo!()
}
