use std::time::Duration;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::{role::Role, user::User};

use super::error::CtxError;

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
