extern crate csv;
extern crate dialoguer;
extern crate geoutils;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use geoutils::Location;
use rand::{thread_rng, Rng};

#[derive(Clone)]
struct Country {
    loc: Location,
    name: String
}

impl Country {
    fn new(latitude: f64, longitude: f64, name: &str) -> Self{
        Country {
            loc: Location::new(latitude, longitude),
            name: String::from(name)
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut countries: Vec<Country> = Vec::new();
    let mut names:     Vec<String>  = Vec::new();

    for result in rdr.records() {
        let record = result?;

        let latitude:  f64 = record.get(1).unwrap().replace(",", ".").parse()?;
        let longitude: f64 = record.get(2).unwrap().replace(",", ".").parse()?;
        let name: &str = record.get(3).unwrap();

        countries.push(Country::new(latitude, longitude, name));
        names.push(String::from(name));
    }


    let mut found = false;
    let mut attempts: u8 = 0;

    let mut rng = thread_rng();
    let target_idx = rng.gen_range(0..countries.len());
    let target  = &countries[target_idx];

    while !found {
        let selection = dialoguer::FuzzySelect::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&names)
            .interact()
            .unwrap();
        if names[selection] == target.name { found = true; }
        else {
            let distance = target.loc.distance_to(&countries[selection].loc)?.meters() / 1000.0;
            println!("Your guess is {} km off.", distance as i64);
            attempts += 1;
        }
    }

    println!("Yay! You guessed the country in {} attempts.", attempts);
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
