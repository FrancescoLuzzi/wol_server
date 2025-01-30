use super::error::{AuthError, CtxError};
use crate::{
    app_state::SharedAppState,
    model::{role::Role, user::User},
};
use axum::{
    extract::{FromRef, FromRequestParts, OptionalFromRequestParts},
    RequestPartsExt as _,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{DateTime, Utc};
use jsonwebtoken::DecodingKey;
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Ctx {
    pub user_id: Uuid,
    pub roles: Vec<Role>,
    pub valid_totp: bool,
    pub exp: Duration,
    pub iat: DateTime<Utc>,
}

// Constructors.
impl Ctx {
    pub fn new(user_id: Uuid, roles: Vec<Role>) -> Self {
        Self {
            user_id,
            roles,
            ..Default::default()
        }
    }
    pub fn valid(&self) -> bool {
        !self.user_id.is_nil()
    }

    pub fn is_admin(&self) -> bool {
        self.roles.contains(&Role::Admin)
    }

    pub fn with_valid_totp(&mut self, totp_status: bool) -> &mut Self {
        self.valid_totp = totp_status;
        self
    }

    pub fn as_refresh(&mut self) -> &mut Self {
        // 30 days
        self.exp = Duration::from_secs(60 * 60 * 24 * 30);
        self.iat = Utc::now();
        self
    }

    pub fn as_auth(&mut self) -> &mut Self {
        // 2 hours
        self.exp = Duration::from_secs(60 * 60 * 2);
        self.iat = Utc::now();
        self
    }

    pub fn with_user(&mut self, user: &User) -> &mut Self {
        self.user_id = user.id;
        self.roles = user.get_roles().unwrap_or_default();
        self
    }

    pub fn from_jwt(token: &str, key: &jsonwebtoken::DecodingKey) -> Result<Self, CtxError> {
        jsonwebtoken::decode::<Self>(token, key, &jsonwebtoken::Validation::default())
            .map_err(|err| CtxError::JwtDecodeError(err))
            .map(|data| data.claims)
    }

    pub fn to_jwt(&self, key: jsonwebtoken::EncodingKey) -> Result<String, CtxError> {
        jsonwebtoken::encode(&jsonwebtoken::Header::default(), self, &key)
            .map_err(|err| CtxError::JwtEncodeError(err))
    }
}

impl Default for Ctx {
    fn default() -> Self {
        Self {
            user_id: Uuid::nil(),
            roles: Vec::new(),
            exp: Duration::from_secs(300),
            iat: Utc::now(),
            valid_totp: false,
        }
    }
}

impl<S> OptionalFromRequestParts<S> for Ctx
where
    S: Send + Sync,
    SharedAppState: FromRef<S>,
{
    type Rejection = CtxError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        if let Some(TypedHeader(Authorization(bearer))) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .ok()
        {
            let token = Ctx::from_jwt(
                bearer.token(),
                &DecodingKey::from_secret(SharedAppState::from_ref(state).hmac_secret.as_bytes()),
            )?;

            Ok(Some(token))
        } else {
            Ok(None)
        }
    }
}

impl<S> FromRequestParts<S> for Ctx
where
    S: Send + Sync,
    SharedAppState: FromRef<S>,
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match <Ctx as OptionalFromRequestParts<S>>::from_request_parts(parts, state).await {
            Ok(res) => res.ok_or(AuthError::MissingCredentials),
            Err(err) => Err(AuthError::from(err)),
        }
    }
}
