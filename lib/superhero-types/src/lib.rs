pub mod fights;
pub mod heroes;
pub mod location;
pub mod villains;

pub mod bindings {
    wit_bindgen::generate!({ generate_all });
}
