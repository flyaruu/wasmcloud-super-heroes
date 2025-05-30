use crate::{
    bindings::hti::superheroes::types::Hero,
    bindings::wasmcloud::postgres::types::ResultRowEntry,
    types::{
        get_i32_from_value, get_i64_from_value, get_optional_string_from_value,
        get_string_from_value,
    },
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SqlHero {
    pub id: i64,
    pub level: i32,
    pub name: String,
    pub other_name: Option<String>,
    pub picture: String,
    pub powers: String,
}

impl From<SqlHero> for Hero {
    fn from(val: SqlHero) -> Self {
        Hero {
            id: val.id,
            level: val.level,
            name: val.name,
            other_name: val.other_name,
            picture: val.picture,
            powers: val.powers,
        }
    }
}

impl From<&Vec<ResultRowEntry>> for SqlHero {
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
        SqlHero {
            id,
            level,
            name,
            other_name,
            picture,
            powers,
        }
    }
}
