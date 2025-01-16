use super::mw_auth::AUTH_COOKIE;
use crate::ctx::Ctx;
use tower_cookies::Cookies;

pub async fn logout(ctx: Ctx, cookies: Cookies) -> anyhow::Result<()> {
    cookies.remove(AUTH_COOKIE.into());
    Ok(())
}
