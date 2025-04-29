use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SqlVillain {
    // id: i64,
    pub level: i32,
    pub name: String,
    // #[serde(skip_serializing_if = "String::is_empty")]
    pub other_name: Option<String>,
    pub picture: String,
    pub powers: String,
}
