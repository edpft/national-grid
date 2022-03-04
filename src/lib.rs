mod bng_error;
pub mod constants;
pub mod coordinate;
pub mod reference;

use coordinate::BngCoordinates;
use reference::Reference;
use std::convert::From;
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn reference_to_coordinates(string: &str) -> JsValue {
    let reference = Reference::from_str(string).unwrap();
    let coordinates = BngCoordinates::from(reference);
    JsValue::from_serde(&coordinates).unwrap()
}

#[wasm_bindgen]
pub fn coordinates_to_reference(eastings: f64, northings: f64) -> String {
    let tuple = (eastings, northings);
    let coordinates = BngCoordinates::try_from(tuple).unwrap();
    let reference = Reference::try_from(coordinates).unwrap();
    reference.to_string()
}
