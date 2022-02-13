mod bng_error;
pub mod reference;
pub mod reference_string;

use reference::Reference;
use reference_string::ReferenceString;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn reference_to_coordinates(string: &str) -> String {
    let reference_string = ReferenceString::from_str(string).unwrap();
    let reference = Reference::from(reference_string);
    let (easting, northing) = reference.to_coordinates();
    format!("({}, {})", easting, northing)
}
