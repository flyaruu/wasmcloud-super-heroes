use serde::{Deserialize, Serialize};

use crate::{
    bindings::{
        hti::superheroes::types::{Location, LocationType},
        wasmcloud::postgres::types::ResultRowEntry,
    },
    types::{get_i64_from_value, get_string_from_value},
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum SqlLocationType {
    #[default]
    NONE,
    CITY,
    PLANET,
    PLACE,
    ISLAND,
    COUNTRY,
    MOON,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SqlLocation {
    pub id: i64,
    pub description: String,
    pub name: String,
    pub picture: String,
    r#type: SqlLocationType, // TODO use enum
}

impl TryFrom<&str> for SqlLocationType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "CITY" => Ok(Self::CITY),
            "PLANET" => Ok(Self::PLANET),
            "PLACE" => Ok(Self::PLACE),
            "ISLAND" => Ok(Self::ISLAND),
            "COUNTRY" => Ok(Self::COUNTRY),
            "MOON" => Ok(Self::MOON),
            _ => Err(format!("Unexpected type: {}", value)),
        }
    }
}

impl From<SqlLocationType> for LocationType {
    fn from(val: SqlLocationType) -> Self {
        match val {
            SqlLocationType::NONE => panic!("Missing location type 'NONE'"),
            SqlLocationType::CITY => LocationType::City,
            SqlLocationType::PLANET => LocationType::Planet,
            SqlLocationType::PLACE => LocationType::Place,
            SqlLocationType::ISLAND => LocationType::Island,
            SqlLocationType::COUNTRY => LocationType::Country,
            SqlLocationType::MOON => LocationType::Moon,
        }
    }
}

impl From<&Vec<ResultRowEntry>> for SqlLocation {
    fn from(row: &Vec<ResultRowEntry>) -> Self {
        let mut id: i64 = 0;
        let mut name = String::new();
        let mut description = String::new();
        let mut picture = String::new();
        let mut r#type = String::new();

        for entry in row {
            match entry.column_name.as_str() {
                "id" => id = get_i64_from_value(&entry.value),
                // "level" => level = get_i32_from_value(&entry.value),
                "name" => name = get_string_from_value(&entry.value),
                "description" => description = get_string_from_value(&entry.value),
                "picture" => picture = get_string_from_value(&entry.value),
                "location_type" => r#type = get_string_from_value(&entry.value),

                _ => panic!("unknown column: {} {:?}", entry.column_name, entry.value),
            }
        }

        SqlLocation {
            id,
            name,
            description,
            picture,
            r#type: r#type.as_str().try_into().unwrap(),
        }
    }
}

impl From<SqlLocation> for Location {
    fn from(val: SqlLocation) -> Self {
        Location {
            id: val.id,
            name: val.name,
            description: val.description,
            picture: val.picture,
            type_: val.r#type.into(),
        }
    }
}
