use super::role::Role;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    roles: String,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub active: bool,
    pub request_date: DateTime<Utc>,
    pub join_date: DateTime<Utc>,
    pub update_date: DateTime<Utc>,
}

impl User {
    pub fn get_roles(&self) -> Result<Vec<Role>, &'static str> {
        self.roles.split("|").map(Role::try_from).collect()
    }

    pub fn set_roles(&mut self, roles: Vec<Role>) {
        let mut roles = roles.into_iter().map(|x| x.into()).collect::<Vec<&str>>();
        roles.dedup();
        self.roles = roles.join("|");
    }
}
