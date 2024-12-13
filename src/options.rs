use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserOptions {
    pub name: String,
    pub user: u32,
}
