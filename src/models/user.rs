use serde::{Deserialize, Serialize};

use crate::models::DboTable;

#[derive(Deserialize, Serialize)]
pub struct User {
    username: String,
}

impl User {
    const TABLE: &'static str = "users";
}

impl Default for User {
    fn default() -> Self {
        User {
            username: String::from("Undefined"),
        }
    }
}

impl DboTable for User {
    fn get_table() -> &'static str {
        User::TABLE
    }
}
