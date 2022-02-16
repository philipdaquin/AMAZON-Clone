pub mod app;
pub mod components;
pub mod route;

use wasm_bindgen::prelude::*;
use crate::app::App;


pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    Ok(())
}
