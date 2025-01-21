use uuid::Uuid;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UserSignupRequest {
    pub user_id: Uuid,
    pub request_text: String,
}
