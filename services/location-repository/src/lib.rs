mod location_repository;
mod locations;
mod types;

pub mod bindings {
    wit_bindgen::generate!({ 
        world: "location-repository-world",
        path: ["../../lib/bindings/wit/"],
        additional_derives: [serde::Serialize, serde::Deserialize],
        pub_export_macro: true,
        with: {
            "wasi:logging/logging@0.1.0-draft": generate,
            "wasmcloud:postgres/types@0.1.1-draft": generate,
            "wasmcloud:postgres/query@0.1.1-draft": generate,
        },
    });
}
