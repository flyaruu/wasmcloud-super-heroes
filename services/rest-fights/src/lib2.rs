use std::str::from_utf8;

use bindings::api::{export, exports::wasi::http::incoming_handler::Guest, wasi::{http::{outgoing_handler, types::{Fields, IncomingBody, IncomingRequest, OutgoingRequest, OutgoingResponse, ResponseOutparam, Scheme}}, logging::logging::{log, Level}}};
use superhero_types::heroes::SqlHero;

struct FightService;

export!(FightService with_types_in bindings::api);

impl Guest for FightService {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        log(Level::Info, "request", "Request received");
        // let data = superhero_types::get_bytes("wasmcloud:8001", "/api/heroes/random_hero").unwrap();
        // let data_string = from_utf8(&data).unwrap().to_owned();
        // // println!("Data: {}", data_string);
        // write_status_message(response_out, data_string, 200);
        // // write_output(response_out, hero);



        let req = outgoing_handler::OutgoingRequest::new(Fields::new());
        req.set_scheme(Some(&Scheme::Https)).unwrap();
        req.set_authority(Some("dog.ceo")).unwrap();
        req.set_path_with_query(Some("/api/breeds/image/random"))
            .unwrap();

        // Perform the API call to dog.ceo, expecting a URL to come back as the response body
        let dog_picture_url = match outgoing_handler::handle(req, None) {
            Ok(resp) => {
                resp.subscribe().block();
                let response = resp
                    .get()
                    .expect("HTTP request response missing")
                    .expect("HTTP request response requested more than once")
                    .expect("HTTP request failed");
                if response.status() == 200 {
                    log(Level::Error, "thing", "request.....");
                    let response_body = response
                        .consume()
                        .expect("failed to get incoming request body");
                    let body = {
                        let mut buf = vec![];
                        let stream = response_body
                            .stream()
                            .expect("failed to get HTTP request response stream");
                        loop {
                            // pick a chunk size
                            let chunk = stream.read(2048);    // or `.blocking_read(...)`
                            match chunk {
                                Ok(chunk) => {
                                    if chunk.is_empty() {
                                        break;
                                    }
                                    buf.extend(chunk);                                    
                                },
                                Err(e) => {
                                    log(Level::Error, "thing", &format!("Error reading stream: {}", e));
                                    break;
                                }
                            };
                        }
                        buf
                    };                    
                    let _trailers = IncomingBody::finish(response_body);
                    // dog_response.message
                    from_utf8(&body).unwrap().to_owned()
                } else {
                    format!("HTTP request failed with status code {}", response.status())
                }
                
            }
            Err(e) => {
                format!("Got error when trying to fetch dog: {}", e)
            }
        };
        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(200).unwrap();

        // Write the headers and then write the body
        let response_body = response.body().unwrap();
        let mut write_stream = response_body.write().unwrap();
        ResponseOutparam::set(response_out, Ok(response));

    }
}

fn get_random_hero()->Result<SqlHero, String> {
    let req = OutgoingRequest::new(Fields::new());
    req.set_scheme(Some(&Scheme::Http)).unwrap();
    req.set_authority(Some("wasmcloud:8001")).unwrap();
    req.set_path_with_query(Some("/api/heroes/random_hero"))
        .unwrap();
    
    superhero_types::get_item("wasmcloud:8001", "/api/heroes/random_hero")
}
