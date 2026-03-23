use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PayloadData {
    pub user_id: String,
    pub action: String,
    pub timestamp: u64,
}