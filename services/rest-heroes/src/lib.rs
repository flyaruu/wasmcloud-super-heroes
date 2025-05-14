mod hero_repository;

use bindings::api::superheroes::host::hero_repository::{get_all_heroes, get_hero, get_random_hero};
// use bindings::api::superheroes::host::hero_repository::
use superhero_types::{write_output, write_status_message};
use bindings::api::exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::{IncomingRequest, ResponseOutparam};
struct HeroFetcher;
impl Guest for HeroFetcher {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        if let Some(path) = request.path_with_query() {
            if path.starts_with("/api/heroes/random_hero") {
                write_output(response_out, &get_random_hero());
                return;
            }
            if path.starts_with("/api/heroes/") {
                let id: String = path.chars().skip("/api/heroes/".len()).collect();
                match id.parse::<i64>() {
                    Ok(id) => write_output(response_out, &get_hero(id)),
                    Err(e) => {
                        write_status_message(response_out, format!("Invalid id: {}", e), 400);
                    }
                };
                return;
            }
            if path.starts_with("/api/heroes") {
                write_output(response_out, &get_all_heroes());
                return;
            }
            write_status_message(response_out, format!("Path not found: {}", path), 404);
        }
    }
}
bindings::api::export!(HeroFetcher with_types_in bindings::api);
