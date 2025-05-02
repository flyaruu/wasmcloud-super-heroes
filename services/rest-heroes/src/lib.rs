// mod bindings {
//     use crate::HeroFetcher;
//     wit_bindgen::generate!({
//         with: {
//             "wasi:clocks/monotonic-clock@0.2.2": generate,
//             "wasi:http/incoming-handler@0.2.2": generate,
//             "wasi:http/types@0.2.2": generate,
//             "wasi:io/error@0.2.2": generate,
//             "wasi:io/poll@0.2.2": generate,
//             "wasi:io/streams@0.2.2": generate,
//         }
//     });
//     export!(HeroFetcher);
// }

use bindings::api::{exports::wasi::http::incoming_handler::Guest, wasi::http::incoming_handler::{IncomingRequest, ResponseOutparam}, wasmcloud::postgres::query::{query, PgValue}};
// use bindings::{exports::wasi::http::incoming_handler::Guest, wasi::http::types::{IncomingRequest, ResponseOutparam}};
// use bindings::{exports::wasi::http::incoming_handler::{Guest, IncomingRequest, ResponseOutparam}, wasi::http::types::{Fields, OutgoingBody, OutgoingResponse}};
use serde::Serialize;
use superhero_types::{heroes::SqlHero, write_output, write_status_message};
// bindings::api::export!(HeroFetcher);

bindings::api::export!(HeroFetcher with_types_in bindings::api);
// bindings::api::export_component!(HeroFetcher);
struct HeroFetcher;
impl Guest for HeroFetcher {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {

        if let Some(path) = request.path_with_query() {
            if path.starts_with("/api/heroes/random_hero") {
                random_hero(response_out);
                return
            }
            if path.starts_with("/api/heroes/") {
                let id: String = path.chars().skip("/api/heroes/".len()).collect();
                match id.parse::<i64>() {
                    Ok(id) => hero(id, response_out),
                    Err(e) => {
                        write_status_message(response_out, format!("Invalid id: {}",e), 400);
                    }
                };
                return
            }
            if path.starts_with("/api/heroes") {
                all_heroes(response_out);
                return
            }
            write_status_message(response_out, format!("Path not found: {}",path), 404);
            return
        }
        // let a= query("select * from Hero",&[]).unwrap();
        // let heroes: Vec<SqlHero> = a.iter().map(|row| {
        //     row.into()
        // }).collect();
        
        // let response = OutgoingResponse::new(Fields::new());
        // response.set_status_code(200).unwrap();

        // // Write the headers and then write the body
        // let response_body = response.body().unwrap();
        // let write_stream = response_body.write().unwrap();
        // ResponseOutparam::set(response_out, Ok(response));
        // // TODO stream the output somehow
        // let val = serde_json::to_string_pretty(&heroes).unwrap();
        // write_stream.write(val.as_bytes()).unwrap();
        // drop(write_stream);
        // OutgoingBody::finish(response_body, None)
        //     .expect("failed to finish response body");
    }
}

fn random_hero(response_out: ResponseOutparam) {
    let rows = query("select * from Hero order by random() limit 1",&[]).unwrap();
    let hero: SqlHero = rows.first().unwrap().into();
    write_output(response_out, &hero);
}

fn hero(id: i64, response_out: ResponseOutparam) {
    let rows = query("select * from Hero where id = $1",&[PgValue::Int8(id)]).unwrap();
    let rows: Vec<SqlHero> = rows.iter().map(|row| row.into()).collect();
    write_output(response_out, rows);
}

fn all_heroes(response_out: ResponseOutparam) {
    let rows = query("select * from Hero",&[]).unwrap();
    let rows: Vec<SqlHero> = rows.iter().map(|row| row.into()).collect();
    write_output(response_out, rows);
}
