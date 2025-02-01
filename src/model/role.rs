// TODO: integrate better into sqlx
// https://github.com/launchbadge/sqlx/issues/2648#issuecomment-1942814011

use std::fmt::Display;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum Role {
    Admin,
    User,
}

impl Role {
    pub fn parse_roles(roles: &str) -> Result<Vec<Self>, &'static str> {
        roles.split("|").map(Role::try_from).collect()
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => f.write_str("admin"),
            Role::User => f.write_str("user"),
        }
    }
}

impl TryFrom<&str> for Role {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "admin" => Ok(Role::Admin),
            "user" => Ok(Role::User),
            _ => Err("invalid role"),
        }
    }
}

impl TryFrom<String> for Role {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl From<Role> for &'static str {
    fn from(val: Role) -> Self {
        match val {
            Role::Admin => "admin",
            Role::User => "user",
        }
    }
}
