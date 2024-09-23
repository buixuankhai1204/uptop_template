use super::{
    request::{
        RequestCreateUser, RequestGetUser, RequestGetUserByPrimaryKey, RequestUpdateUserStatus,
    },
    response::PublicUser,
};
use crate::domain::user::{entity::User, repository::UserRepository};
use std::{future::Future, sync::Arc};
use uptop_core::common::result::AppResult;

pub trait UserAppInterface: Clone + Send + Sync + 'static {
    fn create_user(
        &self,
        req: RequestCreateUser,
    ) -> impl Future<Output = AppResult<PublicUser>> + Send;

    fn find_user_by_id(
        &self,
        query: &RequestGetUserByPrimaryKey,
    ) -> impl Future<Output = AppResult<PublicUser>> + Send;

    fn find_user(
        &self,
        query: &RequestGetUser,
    ) -> impl Future<Output = AppResult<PublicUser>> + Send;

    fn update_user(&self, user: &User) -> impl Future<Output = AppResult<PublicUser>> + Send;

    fn push_new_user_status(
        &self,
        payload: &RequestUpdateUserStatus,
    ) -> impl Future<Output = AppResult<bool>> + Send;

    fn get_full_field_user(
        &self,
        query: &RequestGetUser,
    ) -> impl Future<Output = AppResult<User>> + Send;
}

#[derive(Clone, Debug)]
pub struct UserApp<US>
where
    US: UserRepository,
{
    user_repo: Arc<US>,
}

impl<US> UserApp<US>
where
    US: UserRepository,
{
    pub fn new(user_repo: Arc<US>) -> Self {
        Self { user_repo }
    }
}

impl<US> UserAppInterface for UserApp<US>
where
    US: UserRepository,
{
    async fn create_user(&self, req: RequestCreateUser) -> AppResult<PublicUser> {
        self.user_repo
            .create_user(&User::try_from(req)?)
            .await
            .map(|user| user.try_into())?
    }

    async fn find_user_by_id(&self, query: &RequestGetUserByPrimaryKey) -> AppResult<PublicUser> {
        self.user_repo
            .find_user_by_id(query)
            .await
            .map(|ref user| user.try_into())?
    }

    async fn find_user(&self, query: &RequestGetUser) -> AppResult<PublicUser> {
        self.user_repo
            .find_user(query)
            .await
            .map(|ref user| user.try_into())?
    }

    async fn update_user(&self, user: &User) -> AppResult<PublicUser> {
        self.user_repo
            .update_user(user)
            .await
            .map(|user| user.try_into())?
    }

    async fn push_new_user_status(&self, payload: &RequestUpdateUserStatus) -> AppResult<bool> {
        self.user_repo.push_new_user_status(payload).await
    }

    async fn get_full_field_user(&self, query: &RequestGetUser) -> AppResult<User> {
        self.user_repo.find_user(query).await
    }
}
