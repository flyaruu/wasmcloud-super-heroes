
use bindings::component::wasmcloud::postgres::query::{query, PgValue};
use superhero_types::{villains::SqlVillain, write_output, write_status_message};
use wasi::{exports::http::incoming_handler::{Guest, ResponseOutparam}, http::types::IncomingRequest};

bindings::component::export!(VillainFetcher with_types_in bindings::component);

struct VillainFetcher;
impl Guest for VillainFetcher {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        if let Some(path) = request.path_with_query() {
            if path.starts_with("/api/villains/random_villain") {
                random_villain(response_out);
                return;
            }
            if path.starts_with("/api/villains/") {
                let id: String = path.chars().skip("/api/villains/".len()).collect();
                match id.parse::<i64>() {
                    Ok(id) => villain(id, response_out),
                    Err(e) => {
                        write_status_message(response_out, format!("Invalid id: {}", e), 400);
                    }
                };
                return;
            }
            if path.starts_with("/api/villains") {
                all_villains(response_out);
                return;
            }
            write_status_message(response_out, format!("Path not found: {}", path), 404);
        }
    }
}

fn random_villain(response_out: ResponseOutparam) {
    let rows = query("select * from Villain order by random() limit 1", &[]).unwrap();
    let villain: SqlVillain = rows.first().unwrap().into();
    write_output(response_out, &villain);
}

fn villain(id: i64, response_out: ResponseOutparam) {
    let rows = query("select * from Villain where id = $1", &[PgValue::Int8(id)]).unwrap();
    let rows: Vec<SqlVillain> = rows.iter().map(|row| row.into()).collect();
    write_output(response_out, rows);
}

fn all_villains(response_out: ResponseOutparam) {
    let rows = query("select * from Villain", &[]).unwrap();
    let rows: Vec<SqlVillain> = rows.iter().map(|row| row.into()).collect();
    write_output(response_out, rows);
}
