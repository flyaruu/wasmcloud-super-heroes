use crate::{bindings::exports::hti::superheroes::hero_repository::Guest, heroes::SqlHero, bindings::hti::superheroes::types::Hero, bindings::wasmcloud::postgres::query::{query, PgValue}};



struct HeroRepository;


impl Guest for HeroRepository {
    fn get_hero(id:i64,) -> Option<Hero> {
        let rows = query("select * from Hero where id = $1", &[PgValue::Int8(id)]).unwrap();
        rows.first().map(|row| row.into())
            .map(|hero: SqlHero| hero.into())
    }

    fn get_random_hero() -> Hero {
        let rows = query("select * from Hero order by random() limit 1", &[]).unwrap();
        let hero: SqlHero = rows.first().unwrap().into();
        hero.into()
    }

    fn get_all_heroes() -> Vec::<Hero> {
        let rows = query("select * from Hero", &[]).unwrap();
        rows.iter().map(|row| row.into())
            .map(|hero: SqlHero| hero.into())
            .collect()
    }
}

crate::bindings::export!(HeroRepository with_types_in crate::bindings);
