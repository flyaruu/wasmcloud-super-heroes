mod bindings {
    // use wasi::http::proxy::export;
    use crate::HeroFetcher;
    wit_bindgen::generate!({
        with: {
            "wasi:clocks/monotonic-clock@0.2.2": generate,
            "wasi:http/incoming-handler@0.2.2": generate,
            "wasi:http/types@0.2.2": generate,
            "wasi:io/error@0.2.2": generate,
            "wasi:io/poll@0.2.2": generate,
            "wasi:io/streams@0.2.2": generate,
        }
    });

    // wit_bindgen::generate!({ generate_all });
    export!(HeroFetcher);
}

use std::{io::{Read as _, Write as _}, str::from_utf8};

use bindings::{exports::wasi::http::incoming_handler::{Guest, IncomingRequest, ResponseOutparam}, wasi::http::types::{Fields, OutgoingBody, OutgoingResponse}};
// use bindings::{exports::wasi::http::incoming_handler::{Guest, IncomingRequest, ResponseOutparam}, wasi::{http::types::{Fields, OutgoingBody, OutgoingResponse}, logging::logging::Level::Info}};
// use wasi::{exports::http::incoming_handler::Guest, http::types::{Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam, Scheme}};
// use bindings::{exports::wasi::http::incoming_handler::{Guest, ResponseOutparam}, wasi::http::types::{Fields, IncomingRequest, Scheme}, wasmcloud::postgres::query::query};
// use bindings::exports::wasi::http::incoming_handler::Guest;
// use wasi::http::types::*;
use superhero_types::{bindings::{wasi::logging::logging::log, wasmcloud::postgres::query::{query, PgValue}}, heroes::SqlHero};

struct HeroFetcher;
impl Guest for HeroFetcher {
    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {

        let a= query("select * from Hero",&[]).unwrap();
        let heroes: Vec<SqlHero> = a.iter().map(|row| {
            row.into()
        }).collect();
        // heroes.iter().for_each(|element|{

        // });
        // let row = format!("Hero: {:?}", resp);
        // log(superhero_types::bindings::wasi::logging::logging::Level::Info, "thing", &row);
        

        
        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(200).unwrap();

        // Write the headers and then write the body
        let response_body = response.body().unwrap();
        let mut write_stream = response_body.write().unwrap();
        ResponseOutparam::set(response_out, Ok(response));
        let val = serde_json::to_string_pretty(&heroes).unwrap();

        write_stream.write(val.as_bytes()).unwrap();
        drop(write_stream);

        OutgoingBody::finish(response_body, None).expect("failed to finish response body");




        // response_out.set(Ok(OutgoingResponse::new(Fields::new())));

        // Build a request to dog.ceo which returns a URL at which we can find a doggo
    }
}

