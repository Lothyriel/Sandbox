use mongodb::{Database, bson::doc, error::Error};

use super::dto::{InsertSave, ReadHashedUser, UserSignup};

type DbResult<T> = Result<T, Error>;

pub trait MemoryRepository {
    async fn save(&self, save: InsertSave) -> DbResult<()>;
}

impl MemoryRepository for Database {
    async fn save(&self, save: InsertSave) -> DbResult<()> {
        self.collection("saves").insert_one(save).await?;

        Ok(())
    }
}

pub trait UserRepository {
    async fn register(&self, user: UserSignup) -> DbResult<()>;
    async fn find_user(&self, username: &str) -> DbResult<Option<ReadHashedUser>>;
}

impl UserRepository for Database {
    async fn register(&self, user: UserSignup) -> DbResult<()> {
        self.collection("users").insert_one(user.hashed()).await?;

        Ok(())
    }

    async fn find_user(&self, username: &str) -> DbResult<Option<ReadHashedUser>> {
        let result = self
            .collection("users")
            .find_one(doc! { "username": username })
            .await?;

        Ok(result)
    }
}
