use super::entity::User;
use crate::application::topic::request::{
    RequestGetUser, RequestGetUserByPartitionKey, RequestGetUserByPrimaryKey,
    RequestUpdateUserStatus,
};
use std::future::Future;
use uptop_core::common::result::AppResult;

pub trait UserRepository: Clone + Send + Sync + 'static {
    fn create_user<'c>(&self, user: &'c User) -> impl Future<Output = AppResult<&'c User>> + Send;

    fn find_user_by_id(
        &self,
        query: &RequestGetUserByPrimaryKey,
    ) -> impl Future<Output = AppResult<User>> + Send;

    fn find_user(&self, query: &RequestGetUser) -> impl Future<Output = AppResult<User>> + Send;

    fn find_users(
        &self,
        query: &RequestGetUserByPartitionKey,
    ) -> impl Future<Output = AppResult<Vec<User>>> + Send;

    fn push_new_user_status(
        &self,
        payload: &RequestUpdateUserStatus,
    ) -> impl Future<Output = AppResult<bool>> + Send;

    fn update_user<'u>(&self, user: &'u User) -> impl Future<Output = AppResult<&'u User>> + Send;
}
