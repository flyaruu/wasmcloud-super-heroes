use crate::bindings::wasmcloud::postgres::query::PgValue;
use std::str::from_utf8;

pub fn get_string_from_value(value: &PgValue) -> String {
    match value {
        PgValue::Varchar((_, s)) => from_utf8(s).unwrap().to_owned(),
        PgValue::Text(s) => s.clone(),
        _ => panic!("Invalid type: {:?}", value),
    }
}

pub fn get_optional_string_from_value(value: &PgValue) -> Option<String> {
    match value {
        PgValue::Varchar((_, s)) => Some(from_utf8(s).unwrap().to_owned()),
        PgValue::Null => None,
        _ => panic!("Invalid type: {:?}", value),
    }
}

pub fn get_i32_from_value(value: &PgValue) -> i32 {
    match value {
        PgValue::Int(i) => *i,
        PgValue::BigInt(i) => *i as i32,
        PgValue::Int4(i) => *i,
        PgValue::Int8(i) => *i as i32,
        _ => panic!("Invalid type: {:?}", value),
    }
}

pub fn get_i64_from_value(value: &PgValue) -> i64 {
    match value {
        PgValue::BigInt(i) => *i,
        PgValue::Int(i) => *i as i64,
        PgValue::Int8(i) => *i,
        _ => panic!("Invalid type: {:?}", value),
    }
}
