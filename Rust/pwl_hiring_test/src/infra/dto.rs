use bcrypt::{DEFAULT_COST, hash};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;

use crate::models::Difficulty;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InsertSave {
    #[serde(rename = "userID")]
    pub user_id: ObjectId,
    #[serde(rename = "gameDate")]
    pub game_date: DateTime<Utc>,
    pub failed: i32,
    pub difficulty: Difficulty,
    pub completed: i32,
    #[serde(rename = "timeTaken")]
    pub time_taken: i32,
}

#[derive(serde::Deserialize)]
pub struct UserSignup {
    pub username: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct UserSignin {
    pub username: String,
    pub password: String,
}

impl UserSignup {
    pub fn hashed(self) -> InsertHashedUser {
        let password = hash(self.password, DEFAULT_COST).expect("Failed to hash passwd");

        InsertHashedUser {
            password,
            username: self.username,
        }
    }
}

#[derive(serde::Serialize)]
pub struct InsertHashedUser {
    pub username: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct ReadHashedUser {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub password: String,
}
