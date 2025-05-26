use std::io::Read;

use bindings::{
    exports::wasi::http::incoming_handler::Guest,
    hti::superheroes::location_repository::{get_all_locations, get_location, get_random_location},
};
use serde::{Serialize, de::DeserializeOwned};
use wasi::http::{
    outgoing_handler,
    types::{
        Fields, IncomingBody, IncomingRequest, OutgoingBody, OutgoingRequest, OutgoingResponse,
        ResponseOutparam, Scheme,
    },
};

pub mod bindings {
    wit_bindgen::generate!({
        world: "location-api-world",
        path: ["../../wit/"],
        additional_derives: [serde::Serialize, serde::Deserialize],
        pub_export_macro: true,
        with: {
            "wasi:clocks/monotonic-clock@0.2.4": ::wasi::clocks::monotonic_clock,
            "wasi:http/incoming-handler@0.2.4": generate,
            "wasi:http/types@0.2.4": ::wasi::http::types,
            "wasi:io/error@0.2.4": ::wasi::io::error,
            "wasi:io/poll@0.2.4": ::wasi::io::poll,
            "wasi:io/streams@0.2.4": ::wasi::io::streams,
            "wasi:logging/logging@0.1.0-draft": generate,
        },
    });
}

struct LocationFetcher;
impl Guest for LocationFetcher {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        if let Some(path) = request.path_with_query() {
            if path.starts_with("/api/locations/random_location") {
                write_output(response_out, get_random_location());
                return;
            }
            if path.starts_with("/api/locations/") {
                let id: String = path.chars().skip("/api/locations/".len()).collect();
                match id.parse::<i64>() {
                    Ok(id) => write_output(response_out, get_location(id)),
                    Err(e) => {
                        write_status_message(response_out, format!("Invalid id: {}", e), 400);
                    }
                };
                return;
            }
            if path.starts_with("/api/locations") {
                write_output(response_out, get_all_locations());
                return;
            }
            write_status_message(response_out, format!("Path not found: {}", path), 404);
        }
    }
}

crate::bindings::export!(LocationFetcher with_types_in crate::bindings);

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
        Err(e) => Err(format!(
            "Error in outgoing request: host: {host} path: {path} error: {:?}",
            e
        )),
    }
}
