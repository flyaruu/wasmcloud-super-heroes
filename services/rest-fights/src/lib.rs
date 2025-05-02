use bindings::api::{
    export,
    exports::wasi::http::incoming_handler::Guest,
    wasi::logging::logging::{log, Level},
};
use superhero_types::{fights::{FightResult, Winner}, heroes::SqlHero, location::{self, SqlLocation}, villains::SqlVillain, write_output};
use wasi::http::types::*;

struct FightService;

export!(FightService with_types_in bindings::api);

impl Guest for FightService {
    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {
        log(Level::Info, "request", "Request received");

        let hero = get_random_hero().unwrap();
        let villain = get_random_villain().unwrap();
        let location = get_random_locations().unwrap();
        

        let hero_name = format!("Hero found: {}", hero.name);
        log(Level::Info, "request", &hero_name);

        let result = FightResult::new(Winner::Heroes, &hero, &villain, &location);
        write_output(response_out, &result);
    }
}

fn get_random_hero() -> Result<SqlHero, String> {
    Ok(superhero_types::get_item::<SqlHero>("wasmcloud:8001", "/api/heroes/random_hero")?)
}

fn get_random_locations() -> Result<SqlLocation, String> {
    Ok(superhero_types::get_item::<SqlLocation>("wasmcloud:8003", "/api/locations/random_location")?)
}
fn get_random_villain() -> Result<SqlVillain, String> {
    Ok(superhero_types::get_item::<SqlVillain>("wasmcloud:8002", "/api/villains/random_villain")?)
}