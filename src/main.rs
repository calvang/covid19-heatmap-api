#![feature(proc_macro_hygiene, decl_macro)]

/**
    COVID-19 Heatmap API
    2020-08-29 by Calvin Huang

    Backend Rocket API that regularly collects and stores COVID-19 and location data
**/

#[macro_use] extern crate rocket;

use clokwerk::{Scheduler, TimeUnits};
use std::time::Duration;
use std::process::{Command, Stdio};
use std::env;
use rocket_cors::{CorsOptions, Error};
use rocket::response::content;
mod parser;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/global")]
pub fn get_global_data() -> content::Json<String> {
    content::Json(parser::parse_global())
}

#[get("/us/counties")]
pub fn get_us_counties() -> content::Json<String> {
    content::Json(parser::parse_us_counties())
}

#[get("/brazil/states")]
pub fn get_brazil_states() -> content::Json<String> {
    content::Json(parser::parse_brazil_states())
}

// main function for rocket
fn main() -> Result<(), Error> {
    env::set_var("RUST_BACKTRACE", "1");
    // handle CORS
    let cors = CorsOptions {
        ..Default::default()
    }.to_cors()?;

    // schedule data updates on different threads
    let mut scheduler = Scheduler::new();
    scheduler.every(720.minutes()).run(|| update_local());
    scheduler.every(1.minutes()).run(|| update_global());
    let _thread_handle = scheduler.watch_thread(Duration::from_millis(100));

    rocket::ignite()
        .mount("/", routes![
            index,
            get_global_data,
            get_us_counties,
            get_brazil_states
        ])
        .attach(cors)
        .launch();

    Ok(())
}

// execute update script on local dataset
fn update_local() {
    println!("Running local data collection scripts.");
    Command::new("./src/dataset/updateLocalData.sh")
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to execute local data cycle script");
    println!("Finished running local scripts.");
}

// execute update script on global dataset
fn update_global() {
    println!("Running global data collection scripts.");
    Command::new("./src/dataset/updateGlobalData.sh")
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to execute global data cycle script");
    println!("Finished running global scripts.");
    println!("Reformatting global data.");
    parser::format_global_data();
    println!("Finished formatting global data.");
}
