use std::io::Read;

use bindings::{
    exports::wasi::http::incoming_handler::Guest,
    hti::superheroes::{
        hero_repository::get_random_hero,
        location_repository::get_random_location,
        types::{FightRequest, FightResult, Fighters, Hero, Location, Team, Villain},
        villain_repository::get_random_villain,
    },
    wasi::logging::logging::{log, Level},
};
use serde::Serialize;
use wasi::{clocks::wall_clock, http::types::*};

pub mod bindings {
    wit_bindgen::generate!({
        world: "rest-fight",
        path: ["../../wit/"],
        additional_derives: [serde::Serialize, serde::Deserialize],
        pub_export_macro: true,
        with: {
            "wasi:clocks/monotonic-clock@0.2.4": ::wasi::clocks::monotonic_clock,
            "wasi:clocks/wall-clock@0.2.4": ::wasi::clocks::wall_clock,
            "wasi:http/incoming-handler@0.2.4": generate,
            "wasi:http/types@0.2.4": ::wasi::http::types,
            "wasi:io/error@0.2.4": ::wasi::io::error,
            "wasi:io/poll@0.2.4": ::wasi::io::poll,
            "wasi:io/streams@0.2.4": ::wasi::io::streams,
            "wasi:logging/logging@0.1.0-draft": generate,
        },
    });
}

struct FightService;

// impl bindings::exports::hti::superheroes::perform_fight::Guest for FightService {

//     #[allow(async_fn_in_trait)]
//     fn random_fighters() -> Fighters {
//         random_fighters()
//     }

//     #[allow(async_fn_in_trait)]
//     fn perform_fight(request:FightRequest) -> FightResult {
//         let hero = request.hero;
//         let villain = request.villain;
//         let location = request.location;

//         let now = wasi::clocks::wall_clock::now().seconds;
//         if hero.level > villain.level {
//             FightResult::new(Team::Heroes, &hero, &villain, &location, now)
//         } else {
//             FightResult::new(Team::Villains, &hero, &villain, &location,now)
//         }
//     }
// }

bindings::export!(FightService with_types_in bindings);

impl Guest for FightService {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        // log(
        //     Level::Info,
        //     "Fight request",
        //     format!(
        //         "Request: {:?} == {:?}",
        //         request.method(),
        //         request.path_with_query()
        //     )
        //     .as_str(),
        // );
        if let Some(path) = request.path_with_query() {
            match (request.method(), path.as_str()) {
                (Method::Get, "/api/fights/randomfighters") => {
                    let fighters = random_fighters();
                    write_output(response_out, &fighters);
                }
                (Method::Get, "/api/fights/randomlocation") => {
                    let location = get_random_location();
                    write_output(response_out, &location);
                }
                (Method::Get, "/api/fights/randomfight") => {
                    let fight_result = execute_random_fight();
                    write_output(response_out, &fight_result);
                }
                (Method::Post, "/api/fights") => {
                    let input_stream = request.consume().unwrap();
                    let mut reader = input_stream.stream().unwrap();
                    let mut buf = Vec::new();
                    reader.read_to_end(&mut buf).unwrap();
                    let fight_request: FightRequest = serde_json::from_reader(&*buf).unwrap();
                    let result = execute_fight(
                        &fight_request.hero,
                        &fight_request.villain,
                        &fight_request.location,
                    )
                    .unwrap();
                    write_output(response_out, &result);
                }
                _ => {
                    log(Level::Error, "request", "Invalid request");
                    write_status_message(response_out, "Invalid request".to_owned(), 404);
                }
            }
        } else {
            log(Level::Error, "request", "Path not found");
            write_status_message(response_out, "Path not found".to_owned(), 400);
        }
    }
}

fn random_fighters() -> Fighters {
    let hero = get_random_hero();
    let villain = get_random_villain();
    Fighters { hero, villain }
}

fn execute_fight(
    hero: &Hero,
    villain: &Villain,
    location: &Location,
) -> Result<FightResult, String> {
    let timestamp = wasi::clocks::wall_clock::now().seconds;
    let winner = if hero.level > villain.level {
        Team::Heroes
    } else {
        Team::Villains
    };
    Ok(FightResult::new(
        winner, hero, villain, location, timestamp,
    ))
}

fn execute_random_fight() -> FightResult {
    let hero = get_random_hero();
    let villain = get_random_villain();
    let location = get_random_location();
    let now = wall_clock::now().seconds;
    let winner = if hero.level > villain.level {
        Team::Heroes
    } else {
        Team::Villains
    };
    FightResult::new(winner, &hero, &villain, &location, now)
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
        Self {
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
            winner_team,
            loser_team,
            location: location.clone(),
        }
    }
}
