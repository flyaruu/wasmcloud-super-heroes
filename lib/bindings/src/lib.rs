pub mod api {
    wit_bindgen::generate!({
        generate_all,
        world: "superhero",
        // export_macro_name: "thing",
        pub_export_macro: true,
    });
}

// pub use api::export_component;
