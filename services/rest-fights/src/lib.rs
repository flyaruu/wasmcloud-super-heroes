use std::str::from_utf8;

use bindings::api::{export, exports::wasi::http::incoming_handler::Guest, wasi::logging::logging::{log, Level}};
use superhero_types::{heroes::SqlHero, write_output};
use wasi::http::types::*;

struct FightService;

export!(FightService with_types_in bindings::api);

impl Guest for FightService {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        log(Level::Info, "request", "Request received");
        // let data = superhero_types::get_bytes("wasmcloud:8001", "/api/heroes/random_hero").unwrap();
        // let data_string = from_utf8(&data).unwrap().to_owned();
        // // println!("Data: {}", data_string);
        // write_status_message(response_out, data_string, 200);
        // // write_output(response_out, hero);

        let hero = get_random_hero().unwrap();
        let hero_name = format!("Hero found: {}",hero.name);
        log(Level::Info, "request", &hero_name);

        write_output(response_out, &hero);

    }
}

fn get_random_hero()->Result<SqlHero, String> {
    // let req = OutgoingRequest::new(Fields::new());
    // req.set_scheme(Some(&Scheme::Http)).unwrap();
    // req.set_authority(Some("wasmcloud:8001")).unwrap();
    // req.set_path_with_query(Some("/api/heroes/random_hero"))
    //     .unwrap();
    
    // log(Level::Info, "request", &format!("Creating outgoing request: {:?}", req));
    superhero_types::get_item("wasmcloud:8001", "/api/heroes/random_hero")
}
