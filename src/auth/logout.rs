use super::ctx::Ctx;
use super::mw_auth::AUTH_HEADER;
use tower_cookies::Cookies;

pub async fn logout(_ctx: Ctx, cookies: Cookies) -> anyhow::Result<()> {
    cookies.remove(AUTH_HEADER.into());
    Ok(())
}
