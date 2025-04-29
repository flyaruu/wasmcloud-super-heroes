mod bindings {
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
    export!(HeroFetcher);
}


use bindings::{exports::wasi::http::incoming_handler::{Guest, IncomingRequest, ResponseOutparam}, wasi::http::types::{Fields, OutgoingBody, OutgoingResponse}};
use superhero_types::{bindings::wasmcloud::postgres::query::query, heroes::SqlHero};

struct HeroFetcher;
impl Guest for HeroFetcher {
    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {

        let a= query("select * from Hero",&[]).unwrap();
        let heroes: Vec<SqlHero> = a.iter().map(|row| {
            row.into()
        }).collect();
        
        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(200).unwrap();

        // Write the headers and then write the body
        let response_body = response.body().unwrap();
        let write_stream = response_body.write().unwrap();
        ResponseOutparam::set(response_out, Ok(response));
        let val = serde_json::to_string_pretty(&heroes).unwrap();
        write_stream.write(val.as_bytes()).unwrap();
        drop(write_stream);
        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
    }
}

