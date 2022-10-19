pub(crate) const ACCEPT_TIME_DIFFERENCE: f64 = 1_800.0;

pub(crate) const TOKEN_LIVE_LIFE: f64 = 172_800.0;

pub(crate) const SECRET_LEN: usize = 30;

pub(crate) const SERVER_SIDE_ERROR: &'static str= "SERVER_SIDE_ERROR";

pub(crate) const CLI_SIDE_ERROR: &'static str= "CLI_SIDE_ERROR";

#[cfg(debug_assertions)]
pub(crate) const IPV4: ([u8; 4], u16) = ([127, 0, 0, 1], 4040);

#[cfg(not(debug_assertions))]
pub(crate) const IPV4: ([u8; 4], u16) = ([0, 0, 0, 0], 80);

#[cfg(debug_assertions)]
pub(crate) const CONNECT_WORD: &'static str = "mongodb://root:password@localhost:27017";

#[cfg(not(debug_assertions))]
pub(crate) const CONNECT_WORD: &'static str = "mongodb://root:password@mongo:27017";
