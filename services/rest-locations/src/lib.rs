
use bindings::component::wasmcloud::postgres::query::{query, PgValue};
use superhero_types::{location::SqlLocation, write_output, write_status_message};
use wasi::{exports::http::incoming_handler::Guest, http::types::{IncomingRequest, ResponseOutparam}};

bindings::component::export!(LocationFetcher with_types_in bindings::component);


struct LocationFetcher;
impl Guest for LocationFetcher {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        if let Some(path) = request.path_with_query() {
            if path.starts_with("/api/locations/random_location") {
                random_location(response_out);
                return;
            }
            if path.starts_with("/api/locations/") {
                let id: String = path.chars().skip("/api/locations/".len()).collect();
                match id.parse::<i32>() {
                    Ok(id) => location(id, response_out),
                    Err(e) => {
                        write_status_message(response_out, format!("Invalid id: {}", e), 400);
                    }
                };
                return;
            }
            if path.starts_with("/api/locations") {
                all_locations(response_out);
                return;
            }
            write_status_message(response_out, format!("Path not found: {}", path), 404);
        }
    }
}

fn random_location(response_out: ResponseOutparam) {
    let rows = query("select name,description,picture,location_type::text as location_type from locations order by random() limit 1", &[]).unwrap();
    let hero: SqlLocation = rows.first().unwrap().into();
    write_output(response_out, &hero);
}

fn location(id: i32, response_out: ResponseOutparam) {
    let rows = query("select name,description,picture,location_type::text as location_type from locations where id = $1", &[PgValue::Int4(id)]).unwrap();
    let rows: Vec<SqlLocation> = rows.iter().map(|row| row.into()).collect();
    write_output(response_out, rows);
}

fn all_locations(response_out: ResponseOutparam) {
    let rows = query(
        "select name,description,picture,location_type::text as location_type from locations",
        &[],
    )
    .unwrap();
    let rows: Vec<SqlLocation> = rows.iter().map(|row| row.into()).collect();
    write_output(response_out, rows);
}
