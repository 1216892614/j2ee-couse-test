use std::net::SocketAddr;

pub(super) struct Listen {
    enable: Option<Enable>,
    url: SocketAddr,
}

enum Enable {
    Debug,
    Release,
}

impl Listen {
    pub(super) fn from(enable: String, url: String) -> Self {
        Self {
            enable: match &enable[..] {
                "" => None,
                "Debug" | "debug" | "DEBUG" => Some(Enable::Debug),
                "Release" | "release" | "RELEASE" => Some(Enable::Release),
                _ => Err(()).expect("U can only use listen.enable in Debug mode and Release mode."),
            },
            url: url
                .parse()
                .expect(&format!("\nThis url: <{}> can not be parsed as a URL", url)),
        }
    }
}
