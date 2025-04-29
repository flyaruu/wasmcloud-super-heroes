use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum SqlLocationType {
    CITY,
    PLANET,
    PLACE,
    ISLAND,
    COUNTRY,
    MOON,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SqlLocation {
    // pub id: i64,
    pub description: String,
    pub name: String,
    pub picture: String,
    // r#type: String, // TODO use enum
}
