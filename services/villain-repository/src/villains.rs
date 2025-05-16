use serde::{Deserialize, Serialize};

use crate::{bindings::{hti::superheroes::types::Villain, wasmcloud::postgres::types::ResultRowEntry}, types::{get_i32_from_value, get_i64_from_value, get_optional_string_from_value, get_string_from_value}};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SqlVillain {
    id: i64,
    pub level: i32,
    pub name: String,
    // #[serde(skip_serializing_if = "String::is_empty")]
    pub other_name: Option<String>,
    pub picture: String,
    pub powers: String,
}

impl Into<Villain> for SqlVillain {
    fn into(self) -> Villain {
        Villain {
            id: self.id,
            level: self.level,
            name: self.name,
            other_name: self.other_name,
            picture: self.picture,
            powers: self.powers,
        }
    }
}

impl From<&Vec<ResultRowEntry>> for SqlVillain {
    fn from(row: &Vec<ResultRowEntry>) -> Self {
        let mut id = 0;
        let mut level = 0;
        let mut name = String::new();
        let mut other_name = None;
        let mut picture = String::new();
        let mut powers = String::new();

        for entry in row {
            match entry.column_name.as_str() {
                "id" => id = get_i64_from_value(&entry.value),
                "level" => level = get_i32_from_value(&entry.value),
                "name" => name = get_string_from_value(&entry.value),
                "othername" => other_name = get_optional_string_from_value(&entry.value),
                "picture" => picture = get_string_from_value(&entry.value),
                "powers" => powers = get_string_from_value(&entry.value),
                _ => panic!("unknown column: {} {:?}", entry.column_name, entry.value),
            }
        }

        SqlVillain {
            id,
            level,
            name,
            other_name,
            picture,
            powers,
        }
    }
}
