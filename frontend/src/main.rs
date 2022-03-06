#![recursion_limit = "512"]

#[macro_use]
extern crate dotenv_codegen;

use wasm_bindgen::prelude::*;
use crate::app::App;
extern crate dotenv;
use dotenv::dotenv;
use dotenv_codegen::dotenv;



// const API_URL: &str = &std::env::var("API_URL").expect("You must set an API KEY");

pub mod app;
pub mod components;
pub mod route;
pub mod hooks;
pub mod types;
pub mod services;
pub mod error;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn main() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    Ok(())
}
