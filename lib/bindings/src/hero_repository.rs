    wit_bindgen::generate!({ 
        world: "hero-repository-world",
        path: ["wit/"],
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
