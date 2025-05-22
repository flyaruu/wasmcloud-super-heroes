mod villain_repository;
mod villains;
mod types;

pub mod bindings {
    wit_bindgen::generate!({ 
        world: "villain-repository-world",
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
