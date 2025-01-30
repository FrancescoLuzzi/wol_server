pub mod ctx;
pub mod error;
pub mod logout;
pub mod password;

pub const AUTH_HEADER: &str = "Authorization";
pub const REFRESH_COOKIE: &str = "WOL_REFRESH_TOKEN";
