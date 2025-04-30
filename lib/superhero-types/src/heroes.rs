use std::str::from_utf8;

use bindings::api::wasmcloud::postgres::{query::PgValue, types::ResultRowEntry};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SqlHero {
    id: i64,
    pub level: i32,
    pub name: String,
    pub other_name: Option<String>,
    pub picture: String,
    pub powers: String,
}

fn get_string_from_value(value: &PgValue) -> String {
    match value {
        PgValue::Varchar((_, s)) => from_utf8(&s).unwrap().to_owned(),
        PgValue::Text(s) => s.clone(),
        _ => panic!("Invalid type: {:?}", value),
    }
}

fn get_optional_string_from_value(value: &PgValue) -> Option<String> {
    match value {
        PgValue::Varchar((_, s)) => Some(from_utf8(&s).unwrap().to_owned()),
        PgValue::Null => None,
        _ => panic!("Invalid type: {:?}", value),
    }
}


fn get_i32_from_value(value: &PgValue) -> i32 {
    match value {
        PgValue::Int(i) => *i,
        PgValue::BigInt(i) => *i as i32,
        PgValue::Int4(i) => *i as i32,
        PgValue::Int8(i) => *i as i32,
        _ => panic!("Invalid type: {:?}",value),
    }
}

fn get_i64_from_value(value: &PgValue) -> i64 {
    
    match value {
        PgValue::BigInt(i) => *i,
        PgValue::Int(i) => *i as i64,
        PgValue::Int8(i) => *i as i64,
        _ => panic!("Invalid type: {:?}",value),
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