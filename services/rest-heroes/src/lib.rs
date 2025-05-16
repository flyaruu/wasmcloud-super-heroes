use types::write_output;
use ::wasi::{exports::http::incoming_handler::Guest, http::types::{IncomingRequest, ResponseOutparam}};

mod hero_repository;
mod heroes;
mod types;

pub mod bindings {
    wit_bindgen::generate!({ 
        world: "hero-repository-world",
        path: ["../../lib/bindings/wit/"],
        additional_derives: [serde::Serialize, serde::Deserialize],
        pub_export_macro: true,
        with: {
            // "wasi:clocks/monotonic-clock@0.2.4": ::wasi::clocks::monotonic_clock,
            // "wasi:http/incoming-handler@0.2.4": generate,
            // "wasi:http/outgoing-handler@0.2.4": ::wasi::http::outgoing_handler,
            // "wasi:http/types@0.2.4": ::wasi::http::types,
            // "wasi:io/error@0.2.4": ::wasi::io::error,
            // "wasi:io/poll@0.2.4": ::wasi::io::poll,
            // "wasi:io/streams@0.2.4": ::wasi::io::streams,
            "wasi:logging/logging@0.1.0-draft": generate,
            "wasmcloud:postgres/types@0.1.1-draft": generate,
            "wasmcloud:postgres/query@0.1.1-draft": generate,
            // "hero-repository-world": generate,
        },
    });

}

// struct HeroFetcher;
// impl Guest for HeroFetcher {
//     fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
//         if let Some(path) = request.path_with_query() {
//             if path.starts_with("/api/heroes/random_hero") {
//                 write_output(response_out, &get_random_hero());
//                 return;
//             }
//             if path.starts_with("/api/heroes/") {
//                 let id: String = path.chars().skip("/api/heroes/".len()).collect();
//                 match id.parse::<i64>() {
//                     Ok(id) => write_output(response_out, &get_hero(id)),
//                     Err(e) => {
//                         write_status_message(response_out, format!("Invalid id: {}", e), 400);
//                     }
//                 };
//                 return;
//             }
//             if path.starts_with("/api/heroes") {
//                 write_output(response_out, &get_all_heroes());
//                 return;
//             }
//             write_status_message(response_out, format!("Path not found: {}", path), 404);
//         }
//     }
// }
// bindings::export!(HeroFetcher with_types_in bindings);
// bindings::superhero::export!(HeroFetcher with_types_in bindings::superhero);
// bindings::superheroes::host::hero_repository::export!(HeroFetcher with_types_in bindings::superheroes::host::hero_repository);
// bindings::api::export!(HeroFetcher with_types_in bindings::api);
