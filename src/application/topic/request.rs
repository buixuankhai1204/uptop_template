use crate::domain::user::entity::{UserRole, UserStatus};
use anyhow::bail;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uptop_core::common::{
    result::{AppError, AppResult},
    utils::new_password,
};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RequestCreateUser {
    pub company_id: Option<Vec<String>>,
    #[validate(length(min = 3))]
    pub user_name: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub status: Option<String>,
    pub role: Option<String>,
    pub display_name: Option<String>,
    pub phone_number: Option<String>,
    pub language: Option<String>,
    pub address: Option<String>,
    pub country: String,
    pub region: String,
    pub city: String,
    pub post_code: String,
    pub email_verify_code: Option<String>,
}

impl RequestCreateUser {
    pub fn try_into_domain(self) -> AppResult<Self> {
        match self.validate() {
            Ok(_) => (),
            Err(err) => bail!(AppError::BadRequest {
                msg: err.to_string()
            }),
        };

        let parse_status = UserStatus::parse(self.status.as_deref())?;
        let status = Some(UserStatus::transform(&parse_status));

        let pass_hashed = new_password(&self.password)?;
        let role = Some(UserRole::matching(self.role.as_deref())?);

        Ok(Self {
            company_id: self.company_id,
            user_name: self.user_name,
            email: self.email,
            password: pass_hashed,
            status,
            role,
            display_name: self.display_name,
            phone_number: self.phone_number,
            language: self.language,
            address: self.address,
            country: self.country,
            region: self.region,
            city: self.city,
            post_code: self.post_code,
            email_verify_code: self.email_verify_code,
        })
    }
}

#[derive(Debug, Error)]
pub enum RequestCreateUserError {
    #[error("{name} already exists")]
    UserNameExisted { name: String },
    #[error("{email} already exists")]
    EmailExisted { email: String },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RequestGetUserByPrimaryKey {
    pub country: String,
    pub region: String,
    pub city: String,
    pub user_id: String,
}

impl RequestGetUserByPrimaryKey {
    pub fn try_into_domain(self) -> AppResult<Self> {
        match self.validate() {
            Ok(_) => (),
            Err(err) => bail!(AppError::BadRequest {
                msg: err.to_string()
            }),
        };

        Ok(Self {
            country: self.country,
            region: self.region,
            city: self.city,
            user_id: self.user_id,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RequestGetUserByPartitionKey {
    pub country: String,
    pub region: String,
    pub city: String,
}

impl RequestGetUserByPartitionKey {
    pub fn try_into_domain(self) -> AppResult<Self> {
        match self.validate() {
            Ok(_) => (),
            Err(err) => bail!(AppError::BadRequest {
                msg: err.to_string()
            }),
        };

        Ok(Self {
            country: self.country,
            region: self.region,
            city: self.city,
        })
    }
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestGetUser {
    pub user_name: String,
    pub email: Option<String>,
}

#[derive(Debug, Error)]
pub enum RequestFindUserError {
    #[error("User not found")]
    UserNotFound,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RequestUpdateUser {
    pub company_id: Option<Vec<String>>,
    #[validate(email)]
    pub email: Option<String>,
    pub status: Option<String>,
    pub role: Option<String>,
    pub display_name: Option<String>,
    pub phone_number: Option<String>,
    pub language: Option<String>,
    pub address: Option<String>,
    pub email_verify_code: Option<String>,
    pub password_recovery_code: Option<String>,
    pub password_recovered_at: Option<String>,
    pub email_verified_at: Option<String>,
}

impl RequestUpdateUser {
    pub fn try_into_domain(self) -> AppResult<Self> {
        match self.validate() {
            Ok(_) => (),
            Err(err) => bail!(AppError::BadRequest {
                msg: err.to_string()
            }),
        };

        let parse_status = UserStatus::parse(self.status.as_deref())?;
        let status = Some(UserStatus::transform(&parse_status));

        let role = UserRole::matching(self.role.as_deref())?;

        Ok(Self {
            company_id: self.company_id,
            email: self.email,
            status,
            role: Some(role),
            display_name: self.display_name,
            phone_number: self.phone_number,
            language: self.language,
            address: self.address,
            email_verify_code: self.email_verify_code,
            password_recovery_code: self.password_recovery_code,
            password_recovered_at: self.password_recovered_at,
            email_verified_at: self.email_verified_at,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RequestUpdateUserStatus {
    pub status: String,
    pub country: String,
    pub region: String,
    pub city: String,
    pub user_id: String,
}

impl RequestUpdateUserStatus {
    pub fn try_into_domain(self) -> AppResult<Self> {
        match self.validate() {
            Ok(_) => (),
            Err(err) => bail!(AppError::BadRequest {
                msg: err.to_string()
            }),
        };

        let parse_status = UserStatus::parse(Some(self.status.as_str()))?;
        let status = UserStatus::transform(&parse_status);

        Ok(Self {
            status,
            country: self.country,
            region: self.region,
            city: self.city,
            user_id: self.user_id,
        })
    }
}
