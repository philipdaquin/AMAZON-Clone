extern crate dotenv;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;

extern crate diesel_full_text_search;

pub mod server;
pub mod db;
pub mod models;
pub mod schema;
pub mod handlers;



