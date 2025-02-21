use super::role::Role;
use chrono::NaiveDate;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(serde::Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    roles: String,
    pub password: String,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub active: bool,
    pub force_password_reset: bool,
    pub onboarding_done: bool,
    pub request_date: NaiveDate,
    pub join_date: Option<NaiveDate>,
    pub update_date: Option<NaiveDate>,
    pub totp_secret: Vec<u8>,
}

impl User {
    pub fn get_roles(&self) -> Result<Vec<Role>, &'static str> {
        Role::parse_roles(&self.roles)
    }

    pub fn set_roles(&mut self, roles: Vec<Role>) {
        let mut roles = roles.into_iter().map(|x| x.into()).collect::<Vec<&str>>();
        roles.dedup();
        self.roles = roles.join("|");
    }
}
