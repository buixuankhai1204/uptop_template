use crate::application::topic::request::{RequestCreateUser, RequestUpdateUser};
use anyhow::anyhow;
use charybdis::{
    macros::charybdis_model,
    types::{List, Text, Timestamp, Timeuuid},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uptop_core::common::{result::AppResult, utils::now_timeuuid};

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[charybdis_model(
    table_name = uptop.users,
    partition_keys = [country, region, city],
    clustering_keys = [user_id],
    global_secondary_indexes = [user_id, user_name, email],
    table_options = r#"
        CLUSTERING ORDER BY (user_id DESC);
    "#
)]
pub struct User {
    pub user_id: Timeuuid,
    pub user_name: Text,
    pub display_name: Option<Text>,
    pub email: Text,
    pub password: Text,
    pub status: List<Text>,
    pub role: Text,
    pub phone_number: Option<Text>,
    pub language: Option<Text>,
    pub address: Option<Text>,
    pub country: Text,
    pub region: Text,
    pub city: Text,
    pub post_code: Text,
    pub owners: Option<List<Timeuuid>>,
    pub admins: Option<List<Timeuuid>>,
    pub organizations: Option<List<Timeuuid>>,
    pub active_organization: Option<Timeuuid>,
    pub other_emails: Option<List<Text>>,
    pub email_verify_code: Option<Text>,
    pub email_verified_at: Option<Timestamp>,
    pub password_recovery_code: Option<Text>,
    pub password_recovered_at: Option<Timestamp>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

// Define enum of status for user
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UserStatus {
    Active(ReasonOfStatus),
    Inactive(ReasonOfStatus),
    Disable(ReasonOfStatus),
    Deleted(ReasonOfStatus),
}

impl UserStatus {
    pub fn parse(input: Option<&str>) -> AppResult<UserStatus> {
        match input {
            Some(val) => {
                let parts: Vec<&str> = val.splitn(2, ':').collect();
                match parts.as_slice() {
                    ["Active", reason] => Ok(UserStatus::Active(ReasonOfStatus::parse(reason)?)),
                    ["Inactive", reason] => {
                        Ok(UserStatus::Inactive(ReasonOfStatus::parse(reason)?))
                    }
                    ["Deleted", reason] => Ok(UserStatus::Deleted(ReasonOfStatus::parse(reason)?)),
                    ["Disable", reason] => Ok(UserStatus::Disable(ReasonOfStatus::parse(reason)?)),
                    _ => Err(anyhow!("User status not found!")),
                }
            }
            None => Ok(UserStatus::Inactive(ReasonOfStatus::FirstTimeAccess)),
        }
    }

    pub fn transform(status: &UserStatus) -> String {
        let status_str = match status {
            UserStatus::Active(reason) => {
                format!("Active:{}", ReasonOfStatus::transform(reason))
            }
            UserStatus::Inactive(reason) => {
                format!("Inactive:{}", ReasonOfStatus::transform(reason))
            }
            UserStatus::Disable(reason) => {
                format!("Disable:{}", ReasonOfStatus::transform(reason))
            }
            UserStatus::Deleted(reason) => {
                format!("Deleted:{}", ReasonOfStatus::transform(reason))
            }
        };

        let timeuuid = now_timeuuid();
        format!("{}:{}", status_str, timeuuid)
    }
}

impl Display for UserStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReasonOfStatus {
    FirstTimeAccess,
    ComeBackAccess,
    LoginAgain,
    AfterRegister,
    Logout,
    Spammer,
    LicenseExpired,
    Scammer,
    ViolatePolicy,
    MultipleAccounts,
}

impl ReasonOfStatus {
    pub fn parse(input: &str) -> AppResult<ReasonOfStatus> {
        match input {
            "FirstTimeAccess" => Ok(ReasonOfStatus::FirstTimeAccess),
            "ComeBackAccess" => Ok(ReasonOfStatus::ComeBackAccess),
            "LoginAgain" => Ok(ReasonOfStatus::LoginAgain),
            "AfterRegister" => Ok(ReasonOfStatus::AfterRegister),
            "Logout" => Ok(ReasonOfStatus::Logout),
            "Spammer" => Ok(ReasonOfStatus::Spammer),
            "LicenseExpired" => Ok(ReasonOfStatus::LicenseExpired),
            "Scammer" => Ok(ReasonOfStatus::Scammer),
            "ViolatePolicy" => Ok(ReasonOfStatus::ViolatePolicy),
            "MultipleAccounts" => Ok(ReasonOfStatus::MultipleAccounts),
            _ => Err(anyhow!("User reason of status not found!")),
        }
    }

    pub fn transform(reason: &ReasonOfStatus) -> String {
        match reason {
            ReasonOfStatus::FirstTimeAccess => "FirstTimeAccess".to_owned(),
            ReasonOfStatus::ComeBackAccess => "ComeBackAccess".to_owned(),
            ReasonOfStatus::LoginAgain => "LoginAgain".to_owned(),
            ReasonOfStatus::AfterRegister => "AfterRegister".to_owned(),
            ReasonOfStatus::Logout => "Logout".to_owned(),
            ReasonOfStatus::Spammer => "Spammer".to_owned(),
            ReasonOfStatus::LicenseExpired => "LicenseExpired".to_owned(),
            ReasonOfStatus::Scammer => "Scammer".to_owned(),
            ReasonOfStatus::ViolatePolicy => "ViolatePolicy".to_owned(),
            ReasonOfStatus::MultipleAccounts => "MultipleAccounts".to_owned(),
        }
    }
}

impl Display for ReasonOfStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
    Guest,
    Member,
    Manager,
    Admin,
}

impl UserRole {
    pub fn matching(input: Option<&str>) -> AppResult<String> {
        match input {
            Some(val) => match val {
                "Guest" => Ok(UserRole::Guest.to_string()),
                "Member" => Ok(UserRole::Member.to_string()),
                "Manager" => Ok(UserRole::Manager.to_string()),
                "Admin" => Ok(UserRole::Admin.to_string()),
                _ => Err(anyhow!("User role not found!")),
            },
            None => Ok("Guest".to_owned()),
        }
    }
}

impl Display for UserRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<RequestCreateUser> for User {
    type Error = anyhow::Error;

    fn try_from(value: RequestCreateUser) -> AppResult<Self> {
        let mut user = User {
            user_id: now_timeuuid(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            ..Default::default()
        };
        // user.company_id = Some(value.company_id); // TODO: convert Vec string to Vec Timeuuid
        user.user_name = value.user_name;
        user.email = value.email;
        user.password = value.password;
        user.role = value.role.unwrap();
        user.display_name = value.display_name;
        user.phone_number = value.phone_number;
        user.language = value.language;
        user.address = value.address;
        user.country = value.country;
        user.region = value.region;
        user.city = value.city;
        user.post_code = value.post_code;
        user.email_verify_code = value.email_verify_code;
        user.status.push(value.status.unwrap());

        Ok(user)
    }
}

impl TryFrom<RequestUpdateUser> for User {
    type Error = anyhow::Error;

    fn try_from(_value: RequestUpdateUser) -> AppResult<Self> {
        todo!()
    }
}
