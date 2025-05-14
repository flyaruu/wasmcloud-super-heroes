use bindings::hero_repository::wasmcloud::postgres::types::ResultRowEntry;
use serde::{Deserialize, Serialize};

use crate::get_string_from_value;

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
    r#type: String, // TODO use enum
}

impl From<&Vec<ResultRowEntry>> for SqlLocation {
    fn from(row: &Vec<ResultRowEntry>) -> Self {
        let mut name = String::new();
        let mut description = String::new();
        let mut picture = String::new();

        let mut r#type = String::new();

        for entry in row {
            match entry.column_name.as_str() {
                // "id" => id = get_i64_from_value(&entry.value),
                // "level" => level = get_i32_from_value(&entry.value),
                "name" => name = get_string_from_value(&entry.value),
                "description" => description = get_string_from_value(&entry.value),
                "picture" => picture = get_string_from_value(&entry.value),
                "location_type" => r#type = get_string_from_value(&entry.value),

                _ => panic!("unknown column: {} {:?}", entry.column_name, entry.value),
            }
        }

        SqlLocation {
            name,
            description,
            picture,
            r#type,
        }
    }
}
