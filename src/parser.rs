#![allow(non_snake_case)]

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// global data from the COVID-19 API summary
#[derive(Serialize, Deserialize, Default)]
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

// Default trait for gnerating default values
impl Default for Global {
    fn default () -> Global {
        Global{
            Global: SummaryGlobal::default(),
            Countries: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct FullData {
    Global: SummaryGlobal,
    Countries: Vec<Country>,
    USCounties: Vec<Value>,
    BrazilStates: Vec<Value>,
}


// parse JSON file into a string
fn parse_json(file_path: &str) -> String {
    let mut json_file = File::open(file_path).unwrap();
    let mut json_buffer = String::new();
    json_file.read_to_string(&mut json_buffer).unwrap();
    return json_buffer;
}

// overwrite JSON file or generate new file
fn write_json(data: &str, file_path: &str) {
    let file = Path::new(file_path);
    let path_display = file.display();
    let mut outfile = match File::create(&file) {
        Err(e) => panic!("Failed to create {}: {}", path_display, e),
        Ok(outfile) => outfile,
    };
    match outfile.write_all(data.as_bytes()) {
        Err(e) => panic!("Failed to write to {}: {}", path_display, e),
        Ok(_) => println!(" - Successfully wrote to {}", path_display),
    };
}

// merge COVID-19 API data with known country data
fn merge_global_data(data: &mut Global) {
    // read JSON strings into custom structs
    let summary_data = parse_json("src/dataset/currentGlobalData.json");
    let country_data = parse_json("src/dataset/fullDataSet.json");
    let summary_values: Summary = serde_json::from_str(&summary_data).unwrap();
    let country_values: Vec<CountryInfo> = serde_json::from_str(&country_data).unwrap();

    // iterate over datasets and merge desired fields
    data.Global = summary_values.Global;
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
}

// reformat all data for root endpoint
pub fn format_all_data() {
    let mut global_data = Global::default();
    merge_global_data(&mut global_data);
    let us_counties: Vec<Value> = serde_json::from_str(&parse_us_counties()).unwrap();
    let brazil_states: Vec<Value> = serde_json::from_str(&parse_brazil_states()).unwrap();
    let data = FullData {
        Global: global_data.Global,
        Countries: global_data.Countries,
        USCounties: us_counties,
        BrazilStates: brazil_states,
    };
    write_json(&serde_json::to_string(&data).unwrap(),
        "src/dataset/allAvailableData.json");
}

// reformat COVID-19 API data by merging desired fields with known country data
pub fn format_global_data() {
    let mut data = Global::default();
    merge_global_data(&mut data);
    // write the formatted data to fullGlobalData.json
    write_json(&serde_json::to_string(&data).unwrap(),
        "src/dataset/fullGlobalData.json");
}

// read in full data set to string
pub fn parse_all() -> String {
    return parse_json("src/dataset/allAvailableData.json");
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
