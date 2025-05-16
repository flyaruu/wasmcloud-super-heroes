use crate::{bindings::{exports::hti::superheroes::villain_repository::Guest, hti::superheroes::types::Villain, wasmcloud::postgres::query::{query, PgValue}}, villains::SqlVillain};



struct VillainRepository;


impl Guest for VillainRepository {
    fn get_villain(id:i64,) -> Option<Villain> {
        let rows = query("select * from Villain where id = $1", &[PgValue::Int8(id)]).unwrap();
        rows.first().map(|row| row.into())
            .map(|villain: SqlVillain| villain.into())
    }

    fn get_random_villain() -> Villain {
        let rows = query("select * from Villain order by random() limit 1", &[]).unwrap();
        let villain: SqlVillain = rows.first().unwrap().into();
        villain.into()
    }

    fn get_all_villains() -> Vec::<Villain> {
        let rows = query("select * from Villain", &[]).unwrap();
        rows.iter().map(|row| row.into())
            .map(|villain: SqlVillain| villain.into())
            .collect()
    }
}

crate::bindings::export!(VillainRepository with_types_in crate::bindings);
