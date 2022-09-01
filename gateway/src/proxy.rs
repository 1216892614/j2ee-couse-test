use std::net::SocketAddr;

pub(super) struct Proxy {
    backend: SocketAddr,
    rewrite: String,
}

impl Proxy {
    pub(super) fn from(backend: String, rewrite: String) -> Self {
        Self {
            backend: backend.parse().expect(&format!(
                "\nThis url: <{}> can not be parsed as a backend",
                backend
            )),
            rewrite,
        }
    }
}
