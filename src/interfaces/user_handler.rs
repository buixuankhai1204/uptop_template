use crate::application::topic::{
    app::UserAppInterface,
    request::{RequestCreateUser, RequestCreateUserError, RequestGetUser},
};
use anyhow::bail;
use std::sync::Arc;
use uptop_core::common::result::AppResult;

#[derive(Clone, Debug)]
pub struct UserHandler<UA: UserAppInterface> {
    pub user_app: Arc<UA>,
}

pub async fn on_create_new_user<UA: UserAppInterface>(
    handler: UserHandler<UA>,
    payload: String,
) -> AppResult<String> {
    let body: RequestCreateUser = serde_json::from_str(&payload)?;
    let req = body.try_into_domain()?;

    let is_email_existed = handler
        .user_app
        .find_user(&RequestGetUser {
            email: Some((*req.email).to_string()),
            ..Default::default()
        })
        .await;

    if is_email_existed.is_ok() {
        bail!(RequestCreateUserError::EmailExisted {
            email: (*req.email).to_string()
        })
    }

    let is_user_name_existed = handler
        .user_app
        .find_user(&RequestGetUser {
            user_name: (*req.user_name).to_string(),
            ..Default::default()
        })
        .await;

    if is_user_name_existed.is_ok() {
        bail!(RequestCreateUserError::UserNameExisted {
            name: (*req.user_name).to_string()
        })
    }

    let result = handler.user_app.create_user(req).await?;
    Ok(serde_json::to_string(&result)?)
}

pub async fn on_find_user<UA: UserAppInterface>(
    handler: UserHandler<UA>,
    payload: String,
) -> AppResult<String> {
    let query: RequestGetUser = serde_json::from_str(&payload)?;
    let result = handler.user_app.find_user(&query).await?;
    Ok(serde_json::to_string(&result)?)
}
