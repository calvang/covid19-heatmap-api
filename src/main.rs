#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
// extern crate serde;
// extern crate serde_json;
// Scheduler, and trait for .seconds(), .minutes(), etc.
use clokwerk::{Scheduler, TimeUnits};
// Import week days and WeekDay
// use clokwerk::Interval::*;
// use std::thread;
use std::time::Duration;
use std::process::{Command, Stdio};
use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::env;
use rocket::response::content;
//use rocket_contrib::json::Json;
use rocket_cors::{CorsOptions, Error};

// struct CorsOptions {
//     pub allowed_origins: rocket_cors::AllowedOrigins::all(),
//     pub allowed_methods: rocket_cors::AllowedMethods,
//     pub allowed_headers: rocket_cors::AllowedHeaders,
//     pub allow_credentials: bool,
//     pub expose_headers: HashSet<String>,
//     pub max_age: Option<usize>,
//     pub send_wildcard: bool,
//     pub fairing_route_base: String,
//     pub fairing_route_rank: isize,
// }

// execute update script on dataset
fn run_scripts() {
    println!("Running data collection scripts.");
    Command::new("./src/dataset/updateData.sh")
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to execute data cycle script");
    println!("Finished running scripts.");
}

// read in us county data as JSON
fn parse_us_counties() -> String {
    let mut json_file = File::open("src/dataset/fullCountyData.json").unwrap();
    let mut json_buffer = String::new();
    json_file.read_to_string(&mut json_buffer).unwrap();
    return json_buffer;
    //let json_data: serde_json::Value = serde_json::from_reader(json_file).unwrap();
}

// read in brazil state data as JSON
fn parse_brazil_states() -> String {
    let mut json_file = File::open("src/dataset/brazilStateDataCoords.json").unwrap();
    let mut json_buffer = String::new();
    json_file.read_to_string(&mut json_buffer).unwrap();
    return json_buffer;
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/us/counties")]
fn get_us_counties() -> content::Json<String> {
    content::Json(parse_us_counties())
}

#[get("/brazil/states")]
fn get_brazil_states() -> content::Json<String> {
    content::Json(parse_brazil_states())
}

// main function for rocket
fn main() -> Result<(), Error> {
    env::set_var("RUST_BACKTRACE", "1");
    // handle CORS
    let cors = CorsOptions {
        ..Default::default()
    }.to_cors()?;

    let mut scheduler = Scheduler::new();
    scheduler.every(1.minutes()).run(|| run_scripts());
    let _thread_handle = scheduler.watch_thread(Duration::from_millis(100));
    rocket::ignite()
        .mount("/", routes![index, get_us_counties, get_brazil_states])
        .attach(cors)
        .launch();

    Ok(())
}
