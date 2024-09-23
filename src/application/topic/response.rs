use crate::domain::user::entity::User;
use serde::{Deserialize, Serialize};
use uptop_core::common::result::AppResult;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicUser {
    pub user_id: String,
    pub user_name: String,
    pub display_name: Option<String>,
    pub email: String,
    pub phone_number: Option<String>,
    pub role: String,
    pub language: Option<String>,
    pub address: Option<String>,
    pub country: String,
    pub region: String,
    pub city: String,
    pub post_code: String,
    pub status: String,
    pub other_emails: Option<Vec<String>>,
    pub email_verified_at: Option<String>,
    pub password_recovered_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl TryFrom<&User> for PublicUser {
    type Error = anyhow::Error;

    fn try_from(user: &User) -> AppResult<Self> {
        Ok(Self {
            user_id: user.user_id.to_string(),
            user_name: user.user_name.to_string(),
            display_name: Some(user.display_name.as_deref().unwrap_or_default().to_string()),
            email: user.email.to_string(),
            phone_number: Some(user.phone_number.as_deref().unwrap_or_default().to_string()),
            role: user.role.to_string(),
            language: Some(user.language.as_deref().unwrap_or_default().to_string()),
            address: Some(user.address.as_deref().unwrap_or_default().to_string()),
            country: (*user.country).to_string(),
            region: (*user.region).to_string(),
            city: (*user.city).to_string(),
            post_code: (*user.post_code).to_string(),
            other_emails: Some(user.other_emails.as_deref().unwrap_or_default().to_vec()),
            email_verified_at: user.email_verified_at.map(|value| value.to_string()),
            password_recovered_at: user.password_recovered_at.map(|value| value.to_string()),
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
            status: user
                .status
                .last()
                .expect("Can not find status of user!")
                .to_string(),
        })
    }
}
