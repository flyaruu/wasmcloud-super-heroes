use bindings::api::wasi::http::incoming_handler::ResponseOutparam;
use serde::{de::DeserializeOwned, Serialize};
use wasi::http::{outgoing_handler, types::{Fields, IncomingBody, OutgoingBody, OutgoingRequest, OutgoingResponse, Scheme}};

pub mod fights;
pub mod heroes;
pub mod location;
pub mod villains;

use std::io::Read;

pub fn write_status_message(response_out: ResponseOutparam, message: String, status_code: u16) {
    let response = OutgoingResponse::new(Fields::new());
    response.set_status_code(status_code).unwrap();
    let response_body = response.body().unwrap();
    let write_stream = response_body.write().unwrap();
    ResponseOutparam::set(response_out, Ok(response));
    write_stream.write(message.as_bytes()).unwrap();
    drop(write_stream);
    OutgoingBody::finish(response_body, None)
        .expect("failed to finish response body");
}
pub fn write_output<S: Serialize>(response_out: ResponseOutparam, serializable: S) {
    let headers = Fields::from_list(&[
        ("content-type".to_string(), "application/json".as_bytes().to_vec()),
    ]).unwrap();
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
    OutgoingBody::finish(response_body, None)
        .expect("failed to finish response body");
}

pub fn get_item<D: DeserializeOwned>(host: &str, path: &str) -> Result<D,String> {
    let data = get_bytes(host, path)?;
    serde_json::from_slice(&data).unwrap()
}

pub fn get_bytes(host: &str, path: &str) -> Result<Vec<u8>,String> {
    let req = OutgoingRequest::new(Fields::new());
    req.set_scheme(Some(&Scheme::Http)).unwrap();
    req.set_authority(Some(host)).unwrap();
    req.set_path_with_query(Some(path))
        .unwrap();

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
                Err(format!("HTTP request failed with status code {}", response.status()))
            }
        }
        Err(e) => {
            Err(format!("Got error when trying to fetch dog: {}", e))
        }
    }
}
