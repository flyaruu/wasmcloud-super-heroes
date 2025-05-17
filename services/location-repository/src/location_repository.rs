use crate::{bindings::{exports::hti::superheroes::location_repository::{Guest, Location}, wasmcloud::postgres::query::{query, PgValue}}, locations::SqlLocation};

struct LocationRepository;


impl Guest for LocationRepository {
    fn get_location(id:i64,) -> Option<Location> {
        let rows = query("select id,description,name,picture,location_type::text from Locations where id = $1", &[PgValue::Int8(id)]).unwrap();
        rows.first().map(|row| row.into())
            .map(|hero: SqlLocation| hero.into())
    }

    fn get_random_location() -> Location {
        let rows = query("select id,description,name,picture,location_type::text from Locations order by random() limit 1", &[]).unwrap();
        let location: SqlLocation = rows.first().unwrap().into();
        location.into()
    }

    fn get_all_locations() -> Vec::<Location> {
        let rows = query("select id,description,name,picture,location_type::text from Locations", &[]).unwrap();
        rows.iter().map(|row| row.into())
            .map(|hero: SqlLocation| hero.into())
            .collect()
    }
}

crate::bindings::export!(LocationRepository with_types_in crate::bindings);
