use super::ctx::Ctx;
use super::AUTH_HEADER;
use tower_cookies::Cookies;

pub async fn logout(_ctx: Ctx, cookies: Cookies) -> anyhow::Result<()> {
    cookies.remove(AUTH_HEADER.into());
    Ok(())
}
