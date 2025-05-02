use bindings::api::{export, exports::wasi::http::incoming_handler::Guest, wasi::logging::logging::{log, Level}};
use superhero_types::{heroes::SqlHero, write_output};
use wasi::http::types::*;

struct FightService;

export!(FightService with_types_in bindings::api);

impl Guest for FightService {
    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {
        log(Level::Info, "request", "Request received");

        let hero = get_random_hero().unwrap();
        let hero_name = format!("Hero found: {}",hero.name);
        log(Level::Info, "request", &hero_name);

        // let result = FightResult::new(Winner::Heroes, &hero, villain, location);
        write_output(response_out, &hero);

    }
}

fn get_random_hero()->Result<SqlHero, String> {
     superhero_types::get_item("wasmcloud:8001", "/api/heroes/random_hero")
}
