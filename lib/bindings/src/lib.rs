pub mod api {
    wit_bindgen::generate!({
        generate_all,
        world: "superhero-world",
        // export_macro_name: "thing",
        pub_export_macro: true,
    });
}
