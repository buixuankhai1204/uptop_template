use crate::{
    application::user::request::{
        RequestFindUserError, RequestGetUser, RequestGetUserByPartitionKey,
        RequestGetUserByPrimaryKey, RequestUpdateUserStatus,
    },
    domain::user::{entity::User, repository::UserRepository},
};
use anyhow::anyhow;
use charybdis::{
    operations::{Find, Insert, Update},
    types::Timeuuid,
};
use std::{str::FromStr, vec};
use uptop_core::common::{
    db_types::CassandraCacheSession,
    result::{AppError, AppResult},
};

#[derive(Clone, Debug)]
pub struct UserRepo {
    db: CassandraCacheSession,
}

impl UserRepo {
    pub fn new(db: CassandraCacheSession) -> Self {
        Self { db }
    }

    pub async fn migrate_user_table(&self) -> AppResult<()> {
        let session = self.db.lock().await;
        session.execute_unpaged(CREATE_USER_TABLE_QUERY, ()).await?;
        session.execute_unpaged(CREATE_USER_ID_INDEX, ()).await?;
        session.execute_unpaged(CREATE_USER_EMAIL_INDEX, ()).await?;
        session.execute_unpaged(CREATE_USER_NAME_INDEX, ()).await?;
        Ok(())
    }
}

impl UserRepository for UserRepo {
    async fn create_user<'c>(&self, user: &'c User) -> AppResult<&'c User> {
        let session = self.db.lock().await;
        match user.insert().execute(&session).await {
            Ok(_) => Ok(user),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(anyhow!(AppError::InternalServerError))
            }
        }
    }

    async fn find_user_by_id(&self, query: &RequestGetUserByPrimaryKey) -> AppResult<User> {
        let session = self.db.lock().await;
        let result = User {
            country: (*query.country).to_string(),
            region: (*query.region).to_string(),
            city: (*query.city).to_string(),
            user_id: Timeuuid::from_str(&query.user_id)?,
            ..Default::default()
        }
        .find_by_primary_key()
        .execute(&session)
        .await;

        match result {
            Ok(user) => Ok(user),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(anyhow!(RequestFindUserError::UserNotFound))
            }
        }
    }

    async fn find_user(
        &self,
        request_user_by_username_or_email: &RequestGetUser,
    ) -> AppResult<User> {
        let command = match request_user_by_username_or_email.to_owned().email {
            Some(email) => User::maybe_find_first_by_email(email),
            None => User::maybe_find_first_by_user_name(
                request_user_by_username_or_email.to_owned().user_name,
            ),
        };

        let session = self.db.lock().await;
        let result = command.execute(&session).await;

        match result {
            Ok(user) => match user {
                Some(val) => Ok(val),
                None => Err(anyhow!(RequestFindUserError::UserNotFound)),
            },
            Err(err) => {
                tracing::error!("{err:?}");
                Err(anyhow!(AppError::InternalServerError))
            }
        }
    }

    async fn find_users(&self, query: &RequestGetUserByPartitionKey) -> AppResult<Vec<User>> {
        let session = self.db.lock().await;
        let results = User {
            country: (*query.country).to_string(),
            region: (*query.region).to_string(),
            city: (*query.city).to_string(),
            ..Default::default()
        }
        .find_by_partition_key()
        .execute(&session)
        .await;

        match results {
            Ok(value) => match value.try_collect().await {
                Ok(val) => Ok(val),
                Err(err) => {
                    tracing::error!("{err:?}");
                    Err(anyhow!(AppError::InternalServerError))
                }
            },
            Err(err) => {
                tracing::error!("{err:?}");
                Err(anyhow!(AppError::InternalServerError))
            }
        }
    }

    async fn push_new_user_status(&self, payload: &RequestUpdateUserStatus) -> AppResult<bool> {
        let session = self.db.lock().await;
        match session
            .execute_unpaged(
                User::PUSH_STATUS_IF_EXISTS_QUERY,
                (
                    vec![&payload.status.to_string()],
                    &payload.country,
                    &payload.region,
                    &payload.city,
                    Timeuuid::from_str(&payload.user_id)?,
                ),
            )
            .await
        {
            Ok(_) => Ok(true),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(anyhow!(AppError::InternalServerError))
            }
        }
    }

    async fn update_user<'u>(&self, user: &'u User) -> AppResult<&'u User> {
        let session = self.db.lock().await;
        match user.update().execute(&session).await {
            Ok(_) => Ok(user),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(anyhow!(AppError::InternalServerError))
            }
        }
    }
}

static CREATE_USER_TABLE_QUERY: &str = r#"
    CREATE TABLE IF NOT EXISTS uptop.users (
        user_id timeuuid,
        user_name text,
        display_name text,
        email text,
        password text,
        status list<text>,
        role text,
        phone_number text,
        language text,
        address text,
        country text,
        region text,
        city text,
        post_code text,
        owners list<timeuuid>,
        admins list<timeuuid>,
        organizations list<timeuuid>,
        active_organization timeuuid,
        other_emails list<text>,
        email_verify_code text,
        email_verified_at timestamp,
        password_recovery_code text,
        password_recovered_at timestamp,
        created_at timestamp,
        updated_at timestamp,
        PRIMARY KEY ((country, region, city), user_id)
    ) WITH CLUSTERING ORDER BY (user_id DESC);
"#;

static CREATE_USER_ID_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_user_id_index ON uptop.users (user_id);
"#;

static CREATE_USER_EMAIL_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_email_index ON uptop.users (email);
"#;

static CREATE_USER_NAME_INDEX: &str = r#"
    CREATE INDEX IF NOT EXISTS uptop_user_name_index ON uptop.users (user_name);
"#;
