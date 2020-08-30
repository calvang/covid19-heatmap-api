#![allow(non_snake_case)]

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use serde::{Deserialize, Serialize};

// global data from the COVID-19 API summary
#[derive(Serialize, Deserialize)]
struct SummaryGlobal {
    NewConfirmed: u32,
    TotalConfirmed: u32,
    NewDeaths: u32,
    TotalDeaths: u32,
    NewRecovered: u32,
    TotalRecovered: u32,
}

// country data from the COVID-19 API summary
#[derive(Serialize, Deserialize)]
struct SummaryCountry {
    Country: String,
    CountryCode: String,
    NewConfirmed: u32,
    TotalConfirmed: u32,
    NewDeaths: u32,
    TotalDeaths: u32,
    NewRecovered: u32,
    TotalRecovered: u32,
    Date: String,
}

// compiled data from the COVID-19 API summary
#[derive(Serialize, Deserialize)]
struct Summary {
    Global: SummaryGlobal,
    Countries: Vec<SummaryCountry>,
}

// static country info from precollected source
#[derive(Serialize, Deserialize)]
struct CountryInfo {
    Country: String,
    ISO2: String,
    Lat: Option<String>,
    Lon: Option<String>,
    Population: Option<String>,
}

// country data for formatted JSON response
#[derive(Serialize, Deserialize)]
struct Country {
    Country: String,
    Population: u32,
    Lat: f64,
    Lon: f64,
    NewConfirmed: u32,
    TotalConfirmed: u32,
    ConfirmedPerCapita: u32,
    NewDeaths: u32,
    TotalDeaths: u32,
    DeathsPerCapita: u32,
    ConfirmedDeathRate: f32,
    NewRecovered: u32,
    TotalRecovered: u32,
    PercentRecovered: f32,
    Date: String,
    ISO2: String,
}

// full global data for formatted JSON response
#[derive(Serialize, Deserialize)]
struct Global {
    Global: SummaryGlobal,
    Countries: Vec<Country>,
}

// parse JSON file into a string
fn parse_json(file_path: &str) -> String {
    let mut json_file = File::open(file_path).unwrap();
    let mut json_buffer = String::new();
    json_file.read_to_string(&mut json_buffer).unwrap();
    return json_buffer;
}

// reformat COVID-19 API data by merging desired fields with known country data
pub fn format_global_data() {
    // read JSON strings into custom structs
    let summary_data = parse_json("src/dataset/currentGlobalData.json");
    let country_data = parse_json("src/dataset/fullDataSet.json");
    let summary_values: Summary = serde_json::from_str(&summary_data).unwrap();
    let country_values: Vec<CountryInfo> = serde_json::from_str(&country_data).unwrap();

    // iterate over datasets and merge desired fields
    let mut data = Global {
        Global: summary_values.Global,
        Countries: Vec::new(),
    };
    for i in &country_values {
        for j in &summary_values.Countries {
            if i.Lat != None && i.Population != None
                && i.ISO2.to_string() == j.CountryCode.to_string() {
                let population = i.Population.as_ref().unwrap().parse::<u32>().unwrap();
                let country = Country {
                    Country: i.Country.to_string(),
                    Population: population,
                    Lat: i.Lat.as_ref().unwrap().parse::<f64>().unwrap_or(0.0),
                    Lon: i.Lon.as_ref().unwrap().parse::<f64>().unwrap_or(0.0),
                    NewConfirmed: j.NewConfirmed,
                    TotalConfirmed: j.TotalConfirmed,
                    ConfirmedPerCapita: (1_000_000 as f32 * j.TotalConfirmed as f32
                        / population as f32).round() as u32,
                    NewDeaths: j.NewDeaths,
                    TotalDeaths: j.TotalDeaths,
                    DeathsPerCapita: (1_000_000 as f32 * j.TotalDeaths as f32
                        / population as f32).round() as u32,
                    ConfirmedDeathRate: 100 as f32 * j.TotalDeaths as f32
                        / j.TotalRecovered as f32,
                    NewRecovered: j.NewRecovered,
                    TotalRecovered: j.TotalRecovered,
                    PercentRecovered: 100 as f32 * j.TotalRecovered as f32
                        / j.TotalConfirmed as f32,
                    Date: j.Date.to_string(),
                    ISO2: i.ISO2.to_string(),
                };
                data.Countries.push(country);
            }
        }
    }

    // write the formatted data to fullGlobalData.json
    let file_path = Path::new("src/dataset/fullGlobalData.json");
    let path_display = file_path.display();
    let mut outfile = match File::create(&file_path) {
        Err(e) => panic!("Failed to create {}: {}", path_display, e),
        Ok(outfile) => outfile,
    };
    match outfile.write_all(&serde_json::to_string(&data).unwrap().as_bytes()) {
        Err(e) => panic!("Failed to write to {}: {}", path_display, e),
        Ok(_) => println!("successfully wrote to {}", path_display),
    };
}

// read in global data to string
pub fn parse_global() -> String {
    return parse_json("src/dataset/fullGlobalData.json");
}

// read in us county data to string
pub fn parse_us_counties() -> String {
    return parse_json("src/dataset/fullCountyData.json");
}

// read in brazil state data to string
pub fn parse_brazil_states() -> String {
    return parse_json("src/dataset/brazilStateDataCoords.json");
}
