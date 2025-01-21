// TODO: integrate better into sqlx
// https://github.com/launchbadge/sqlx/issues/2648#issuecomment-1942814011

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum Role {
    Admin,
    User,
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

impl Into<&'static str> for Role {
    fn into(self) -> &'static str {
        match self {
            Role::Admin => "admin",
            Role::User => "user",
        }
    }
}
