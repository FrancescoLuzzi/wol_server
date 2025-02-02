use super::REFRESH_COOKIE;
use tower_cookies::cookie::time::Duration;
use tower_cookies::{Cookie, Cookies};

pub async fn logout(cookies: Cookies) -> anyhow::Result<()> {
    let refresh_cookie = Cookie::build((REFRESH_COOKIE, ""))
        .max_age(Duration::seconds(0))
        .http_only(true)
        .path("/")
        .build();
    cookies.add(refresh_cookie);
    Ok(())
}
