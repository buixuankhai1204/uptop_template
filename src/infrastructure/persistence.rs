use uptop_core::common::{db_types::CassandraCacheSession, result::AppResult};

pub(crate) mod user_repository;

#[derive(Debug)]
pub struct IDRepositories {
    pub user: user_repository::UserRepo,
}

impl IDRepositories {
    pub fn new(session: CassandraCacheSession) -> Self {
        Self {
            user: user_repository::UserRepo::new(session),
        }
    }

    pub async fn auto_mod_identification_migrate(&self) -> AppResult<()> {
        self.user.migrate_user_table().await?;
        Ok(())
    }
}
