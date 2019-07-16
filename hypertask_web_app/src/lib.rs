#[macro_use]
extern crate serde_derive;
extern crate hypertask_engine;
extern crate serde_json;
extern crate wasm_bindgen;

use hypertask_engine::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

fn run_command(
    command: Result<Command, serde_json::error::Error>,
) -> Result<Vec<FinalisedTask>, String> {
    Err(String::from("whoops, something's gone wrong"))
}

#[wasm_bindgen]
pub fn run_command_from_js(command_wrapped: &JsValue) -> JsValue {
    JsValue::from_serde(&run_command(command_wrapped.into_serde())).unwrap()
}

//import { send_example_to_js, receive_example_from_js } from "example";

//// Get the example object from wasm.
//let example = send_example_to_js();

//// Add another "Vec" element to the end of the "Vec<Vec<f32>>"
//example.field2.push([5,6]);

//// Send the example object back to wasm.
//receive_example_from_js(example);
