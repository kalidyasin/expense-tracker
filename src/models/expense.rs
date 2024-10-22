use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Expense {
    pub id: i32,
    pub user_id: i32,
    pub amount: f64,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}
