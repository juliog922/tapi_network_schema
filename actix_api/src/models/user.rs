use serde::{Deserialize, Serialize};

use sqlx::FromRow;

// ==== Database ====

#[derive(FromRow, Debug)]
pub struct UserDB {
    pub id: String
}

// ==== Core ====

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct User {
    pub id: String
}

impl From<User> for UserDB {
    fn from(user: User) -> Self {
        Self {
            id: user.id
        }
    }
}