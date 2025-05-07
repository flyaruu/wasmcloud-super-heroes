use std::io::Read;

use bindings::api::{
    export,
    exports::wasi::http::incoming_handler::Guest,
    wasi::logging::logging::{log, Level},
};
use superhero_types::{
    fights::{FightRequest, FightResult, Fighters, Winner},
    heroes::SqlHero,
    location::SqlLocation,
    villains::SqlVillain,
    write_output, write_status_message,
};
use wasi::http::types::*;

struct FightService;

export!(FightService with_types_in bindings::api);

impl Guest for FightService {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        if let Some(path) = request.path_with_query() {
            match (request.method(), path.as_str()) {
                (Method::Get, "/api/fights/randomfighters") => {
                    let fighters = random_fighters();
                    write_output(response_out, &fighters);
                    return;
                },
                (Method::Get, "/api/fights/randomlocation") => {
                    let location = get_random_location();
                    match location {
                        Ok(location) => write_output(response_out, &location),
                        Err(e) => {
                            let error_msg = format!("Error: {}", e);
                            log(Level::Error, "request", &error_msg);
                            write_status_message(response_out, error_msg, 500);
                        },
                    }
                    return;
                },
                (Method::Get, "/api/fights/randomfight") => {
                    let fight_result = execute_random_fight();
                    match fight_result {
                        Ok(result) => write_output(response_out, &result),
                        Err(e) => {
                            let error_msg = format!("Error: {}", e);
                            log(Level::Error, "request", &error_msg);
                            write_status_message(response_out, error_msg, 500);
                        },
                    }
                },
                (Method::Post,"/api/fights") => {
                    let input_stream = request.consume().unwrap();
                    let mut reader = input_stream.stream().unwrap();
                    let mut buf = Vec::new();
                    reader.read_to_end(&mut buf).unwrap();
                    let fight_request: FightRequest = serde_json::from_reader(&*buf).unwrap();
                    let result = execute_fight(
                        &fight_request.hero,
                        &fight_request.villain,
                        &fight_request.location,
                    ).unwrap();
                    write_output(response_out, &result);
                    return;
                },
                _ => {
                    log(Level::Error, "request", "Invalid request");
                    write_status_message(response_out, "Invalid request".to_owned(), 404);
                    return;
                }                
            }
        } else {
            log(Level::Error, "request", "Path not found");
            write_status_message(response_out, "Path not found".to_owned(), 400);
            return;
        }
    }
}

fn random_fighters()->Fighters {
    let hero = get_random_hero().unwrap();
    let villain = get_random_villain().unwrap();
    Fighters {hero, villain}
}

fn execute_fight(hero: &SqlHero, villain: &SqlVillain, location: &SqlLocation) -> Result<FightResult, String> {
    let winner = if hero.level > villain.level {
        Winner::Heroes
    } else {
        Winner::Villains
    };
    Ok(FightResult::new(winner, &hero, &villain, &location))
}


fn execute_random_fight() -> Result<FightResult, String> {
    let hero = get_random_hero()?;
    let villain = get_random_villain()?;
    let location = get_random_location()?;
    let winner = if hero.level > villain.level {
        Winner::Heroes
    } else {
        Winner::Villains
    };
    Ok(FightResult::new(winner, &hero, &villain, &location))
}



fn get_random_hero() -> Result<SqlHero, String> {
    superhero_types::get_item::<SqlHero>("wasmcloud:8001", "/api/heroes/random_hero")
}

fn get_random_location() -> Result<SqlLocation, String> {
    superhero_types::get_item::<SqlLocation>("wasmcloud:8003", "/api/locations/random_location")
}
fn get_random_villain() -> Result<SqlVillain, String> {
    superhero_types::get_item::<SqlVillain>("wasmcloud:8002", "/api/villains/random_villain")
}
