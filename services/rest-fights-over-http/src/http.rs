use std::io::Read;

use serde::de::DeserializeOwned;
use wasi::http::{
    outgoing_handler,
    types::{Fields, IncomingBody, OutgoingRequest, Scheme},
};

use crate::bindings::{
    hti::superheroes::types::{Hero, Location, Villain},
    wasi::logging::logging::{log, Level},
};

pub(crate) fn get_random_hero() -> Result<Hero, String> {
    get_item::<Hero>("wasmcloud:8001", "/api/heroes/random_hero")
}

pub(crate) fn get_random_location() -> Result<Location, String> {
    get_item::<Location>("wasmcloud:8003", "/api/locations/random_location")
}
pub(crate) fn get_random_villain() -> Result<Villain, String> {
    get_item::<Villain>("wasmcloud:8002", "/api/villains/random_villain")
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
        Err(e) => Err(format!(
            "Error in outgoing request: host: {host} path: {path} error: {:?}",
            e
        )),
    }
}
