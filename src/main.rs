#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
// Scheduler, and trait for .seconds(), .minutes(), etc.
use clokwerk::{Scheduler, TimeUnits};
// Import week days and WeekDay
// use clokwerk::Interval::*;
// use std::thread;
use std::time::Duration;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/us/counties")]
fn get_us_counties() -> &'static str {
    "Hello, world!"
}

#[get("/brazil")]
fn get_brazil_states() -> &'static str {
    "Hello, world!"
}

fn main() {
    let mut scheduler = Scheduler::new();
    scheduler.every(360.minutes()).run(|| println!("Periodic task"));
    let thread_handle = scheduler.watch_thread(Duration::from_millis(100));
    rocket::ignite().mount("/", routes![index]).launch();
    rocket::ignite().mount("/", routes![get_us_counties]).launch();
    rocket::ignite().mount("/", routes![get_brazil_states]).launch();
}