use bindings::{hti::superheroes::villain_repository::{get_all_villains, get_random_villain, get_villain}};
use superhero_types::{write_output, write_status_message};
use wasi::{exports::http::incoming_handler::Guest, http::types::{IncomingRequest, ResponseOutparam}};
mod villain_repository;

struct VillainFetcher;
impl Guest for VillainFetcher {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        if let Some(path) = request.path_with_query() {
            if path.starts_with("/api/villains/random_villain") {
                write_output(response_out, &get_random_villain());
                return;
            }
            if path.starts_with("/api/villains/") {
                let id: String = path.chars().skip("/api/villains/".len()).collect();
                match id.parse::<i64>() {
                    Ok(id) => {
                        write_output(response_out, &get_villain(id));
                    }, 
                    Err(e) => {
                        write_status_message(response_out, format!("Invalid id: {}", e), 400);
                    }
                };
                return;
            }
            if path.starts_with("/api/villains") {
                write_output(response_out, get_all_villains());
                return;
            }
            write_status_message(response_out, format!("Path not found: {}", path), 404);
        }
    }
}

bindings::export!(VillainFetcher with_types_in bindings);