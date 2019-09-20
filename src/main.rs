extern crate clap;
extern crate indicatif;
#[macro_use]
extern crate json;

use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use json::JsonValue;
use std::fs;
use std::fs::File;

// Start of the command line tool
fn main() {
    println!("Running command");

    // Build the args list
    let matches = App::new("csv-to-json")
        .version("0.1.0")
        .author("Justin Rhoades")
        .about("Convert csv to json")
        .arg(
            Arg::with_name("csv")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("path to csv file"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Sets path to output file")
                .takes_value(true),
        )
        .get_matches();

    // Get the arguements
    let input: &str = matches.value_of("csv").unwrap();
    let output: &str = matches.value_of("output").unwrap_or("./jsonResult.json");

    // Open the input file
    let file = File::open(input).expect("Failed to open file");

    // Create a json body
    let mut json_body: JsonValue;

    // Make it an array
    json_body = array![];

    // Create reader and extract the header information
    let mut rdr = csv::Reader::from_reader(file);
    let headers = rdr.headers().expect("Failed to read headers").clone();

    // Count
    let count = 198469;

    // Set the progress bar
    let bar = ProgressBar::new(count);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .progress_chars("#>-"),
    );

    // Loop over the records and create an object per record and add it to the array
    for result in rdr.records() {
        bar.inc(1);
        let record = result.unwrap();
        let mut element = object! {};
        for index in 0..headers.len() {
            if index >= record.len() {
                break;
            }

            let header: &str = &headers[index];
            let value: &str = &record[index];

            if value.is_empty() {
                element[header] = json::Null;
            } else {
                element[header] = value.into();
            }
        }
        json_body
            .push(element.clone())
            .expect("Failed to push element into json");
    }

    // Write the json data to the file
    fs::write(output, json::stringify_pretty(json_body, 4)).expect("Failed to write file");

    // Complete the progress bar
    bar.finish_with_message("Conversion completed");
}
