use std::io::Read;


use bindings::{exports::{hti::superheroes::perform_fight::{FightRequest, FightResult}, wasi::http::incoming_handler::Guest}, hti::superheroes::types::{Hero, Location, Team, Villain}, wasi::logging::logging::{log, Level}};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use wasi::{http::{outgoing_handler, types::*}};

pub mod bindings {
    wit_bindgen::generate!({ 
        world: "fight-api-world",
        path: ["../../lib/bindings/wit/"],
        additional_derives: [serde::Serialize, serde::Deserialize],
        pub_export_macro: true,
        with: {
            "wasi:clocks/monotonic-clock@0.2.4": ::wasi::clocks::monotonic_clock,
            "wasi:clocks/wall-clock@0.2.4": ::wasi::clocks::wall_clock,
            "wasi:http/incoming-handler@0.2.4": generate,
            "wasi:http/outgoing-handler@0.2.4": ::wasi::http::outgoing_handler,
            "wasi:http/types@0.2.4": ::wasi::http::types,
            "wasi:io/error@0.2.4": ::wasi::io::error,
            "wasi:io/poll@0.2.4": ::wasi::io::poll,
            "wasi:io/streams@0.2.4": ::wasi::io::streams,
            "wasi:logging/logging@0.1.0-draft": generate,
        },
    });
}



struct FightService;

impl bindings::exports::hti::superheroes::perform_fight::Guest for FightService {

    #[allow(async_fn_in_trait)]
    fn random_fighters() -> (String, String) {
        // let hero = get_random_hero().unwrap();
        // let villain = get_random_villain().unwrap();
        // (hero, villain)
        ("hero".to_string(), "villain".to_string())
    }

    #[allow(async_fn_in_trait)]
    fn perform_fight(request:FightRequest) -> FightResult {
        let hero = request.hero;
        let villain = request.villain;
        let location = request.location;

        let now = wasi::clocks::wall_clock::now().seconds;
        let id = uuid::Uuid::new_v4().to_string();

        if hero.level > villain.level {
            FightResult {
                id: id,
                fight_date: now,
                winner_name: hero.name.clone(),
                winner_level: hero.level,
                winner_powers: hero.powers.clone(),
                winner_picture: hero.picture.clone(),
                loser_name: villain.name.clone(),
                loser_level: villain.level,
                loser_powers: villain.powers.clone(),
                loser_picture: villain.picture.clone(),
                winner_team: Team::Heroes,
                loser_team: todo!(),
                location,
            }
        } else {
            FightResult::new(Team::Villains, &hero, &villain, &location,now)
        }
    }
}

bindings::export!(FightService with_types_in bindings);

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

fn random_fighters()->(Hero,Villain) {
    // bindings::hti::superheroes::types::Fighters;
    let hero = get_random_hero().unwrap();
    let villain = get_random_villain().unwrap();
    (hero, villain)
}

fn execute_fight(hero: &Hero, villain: &Villain, location: &Location) -> Result<FightResult, String> {
    let timestamp = wasi::clocks::wall_clock::now().seconds;
    let winner = if hero.level > villain.level {
        Team::Heroes
    } else {
        Team::Villains
    };
    Ok(FightResult::new(winner, &hero, &villain, &location, timestamp))
}


fn execute_random_fight() -> Result<FightResult, String> {
    let hero = get_random_hero()?;
    let villain = get_random_villain()?;
    let location = get_random_location()?;
    let winner = if hero.level > villain.level {
        Team::Heroes
    } else {
        Team::Villains
    };
    Ok(FightResult::new(winner, &hero, &villain, &location))
}



fn get_random_hero() -> Result<Hero, String> {
    get_item::<Hero>("wasmcloud:8001", "/api/heroes/random_hero")
}

fn get_random_location() -> Result<Location, String> {
    get_item::<Location>("wasmcloud:8003", "/api/locations/random_location")
}
fn get_random_villain() -> Result<Villain, String> {
    get_item::<Villain>("wasmcloud:8002", "/api/villains/random_villain")
}

pub fn write_status_message(response_out: ResponseOutparam, message: String, status_code: u16) {
    let response = OutgoingResponse::new(Fields::new());
    response.set_status_code(status_code).unwrap();
    let response_body = response.body().unwrap();
    let write_stream = response_body.write().unwrap();
    ResponseOutparam::set(response_out, Ok(response));
    write_stream.write(message.as_bytes()).unwrap();
    drop(write_stream);
    OutgoingBody::finish(response_body, None).expect("failed to finish response body");
}
pub fn write_output<S: Serialize>(response_out: ResponseOutparam, serializable: S) {
    let headers = Fields::from_list(&[(
        "content-type".to_string(),
        "application/json".as_bytes().to_vec(),
    )])
    .unwrap();
    let response = OutgoingResponse::new(headers);
    response.set_status_code(200).unwrap();
    // Write the headers and then write the body
    let response_body = response.body().unwrap();
    let write_stream = response_body.write().unwrap();
    ResponseOutparam::set(response_out, Ok(response));
    // TODO stream the output somehow
    let val = serde_json::to_string_pretty(&serializable).unwrap();
    write_stream.write(val.as_bytes()).unwrap();
    drop(write_stream);
    OutgoingBody::finish(response_body, None).expect("failed to finish response body");
}

pub fn get_item<D: DeserializeOwned>(host: &str, path: &str) -> Result<D, String> {
    let data = get_bytes(host, path)?;
    serde_json::from_slice(&data).map_err(|e| format!("Failed to parse response: {}", e))
}

pub fn get_bytes(host: &str, path: &str) -> Result<Vec<u8>, String> {
    let req = OutgoingRequest::new(Fields::new());
    req.set_scheme(Some(&Scheme::Http)).unwrap();
    req.set_authority(Some(host)).unwrap();
    req.set_path_with_query(Some(path)).unwrap();

    log(
        Level::Info,
        "request",
        &format!("Creating outgoing request2: {:?}", req),
    );
    match outgoing_handler::handle(req, None) {
        Ok(resp) => {
            resp.subscribe().block();
            let response = resp
                .get()
                .expect("HTTP request response missing")
                .expect("HTTP request response requested more than once")
                .expect("HTTP request failed");
            if response.status() == 200 {
                let response_body = response
                    .consume()
                    .expect("failed to get incoming request body");
                let body = {
                    let mut buf = vec![];
                    let mut stream = response_body
                        .stream()
                        .expect("failed to get HTTP request response stream");
                    stream
                        .read_to_end(&mut buf)
                        .expect("failed to read value from HTTP request response stream");
                    buf
                };
                let _trailers = IncomingBody::finish(response_body);
                Ok(body)
            } else {
                Err(format!(
                    "HTTP request failed with status code {}",
                    response.status()
                ))
            }
        }
        Err(e) => Err(format!("Got error when trying to fetch dog: {}", e)),
    }
}

impl FightResult {
    pub fn new(
        winner_team: Team,
        hero: &Hero,
        villain: &Villain,
        location: &Location,
        timestamp: u64,
    ) -> Self {
        let id = uuid::Uuid::new_v4();
        let winner_name = match winner_team {
            Team::Heroes => hero.name.clone(),
            Team::Villains => villain.name.clone(),
        };
        let winner_level = match winner_team {
            Team::Heroes => hero.level,
            Team::Villains => villain.level,
        };
        let winner_powers = match winner_team {
            Team::Heroes => hero.powers.clone(),
            Team::Villains => villain.powers.clone(),
        };
        let winner_picture = match winner_team {
            Team::Heroes => hero.picture.clone(),
            Team::Villains => villain.picture.clone(),
        };
        let loser_team = match winner_team {
            Team::Heroes => Team::Villains,
            Team::Villains => Team::Heroes,
        };
        let loser_name = match winner_team {
            Team::Heroes => villain.name.clone(),
            Team::Villains => hero.name.clone(),
        };
        let loser_level = match winner_team {
            Team::Heroes => villain.level,
            Team::Villains => hero.level,
        };
        let loser_powers = match winner_team {
            Team::Heroes => villain.powers.clone(),
            Team::Villains => hero.powers.clone(),
        };
        let loser_picture = match winner_team {
            Team::Heroes => villain.picture.clone(),
            Team::Villains => hero.picture.clone(),
        };
        // let winner_team = match winner {
        //     Team::Heroes => "heroes",
        //     Team::Villains => "villains",
        // };
        // let loser_team = match winner {
        //     Team::Villains => "heroes",
        //     Team::Heroes => "villains",
        // };

        FightResult {
            id: id.to_string(),
            fight_date: timestamp,
            winner_name,
            winner_level,
            winner_powers,
            winner_picture,
            loser_name,
            loser_level,
            loser_powers,
            loser_picture,
            winner_team: winner_team,
            loser_team: loser_team,
            location: location.clone(),
        }
    }
}
