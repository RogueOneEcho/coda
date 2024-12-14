use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct UserOptions {
    pub name: String,
    pub user: u32,
}
