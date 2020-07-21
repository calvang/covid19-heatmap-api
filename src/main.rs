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
use std::process::Command;
use std::fs::File;
use std::io::Read;
//use rocket::response::content;
use rocket_contrib::json::Json;

fn run_scripts() {
    Command::new("./dataset/updateData.sh")
        .arg("")
        .output()
        .expect("Failed to execute data cycle script");
}

fn parse_counties() -> String {
    let mut json_file = File::open("dataset/fullCountyData.json").unwrap();
    let mut json_buffer = String::new();
    json_file.read_to_string(&mut json_buffer).unwrap();
    return json_buffer;
    //let json_data: serde_json::Value = serde_json::from_reader(json_file).unwrap();
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/us/counties")]
fn get_us_counties() -> Json<String> {
    Json(parse_counties())
}

#[get("/brazil")]
fn get_brazil_states() -> &'static str {
    "Hello, world!"
}

fn main() {
    let mut scheduler = Scheduler::new();
    scheduler.every(360.minutes()).run(|| run_scripts());
    let _thread_handle = scheduler.watch_thread(Duration::from_millis(100));
    rocket::ignite().mount("/", routes![index, get_us_counties, get_brazil_states])
        .launch();
    // rocket::ignite().mount("/us/counties", routes![get_us_counties]).launch();
    // rocket::ignite().mount("/brazil", routes![get_brazil_states]).launch();
}