mod hero_repository;
mod heroes;
mod types;

pub mod bindings {
    wit_bindgen::generate!({
        world: "hero-repository-world",
        path: ["../../wit/"],
        additional_derives: [serde::Serialize, serde::Deserialize],
        pub_export_macro: true,
        with: {
            "wasi:logging/logging@0.1.0-draft": generate,
            "wasmcloud:postgres/types@0.1.1-draft": generate,
            "wasmcloud:postgres/query@0.1.1-draft": generate,
        },
    });
}
