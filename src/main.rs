mod parser;
mod score;
mod validation;

use crate::parser::{parse_input, parse_output};
use crate::score::score;
use crate::validation::validate_output;

extern crate clap;
extern crate nom;
use clap::{App, Arg};

use nom::error::convert_error;
use std::fs;

fn main() -> anyhow::Result<()> {
    let matches = App::new("score-calculator")
        .version("1.0")
        .author("PicoJr")
        .about("Validate output and compute score")
        .arg(
            Arg::with_name("input")
                .value_name("INPUT")
                .help("Sets input file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .value_name("OUTPUT")
                .help("Sets output file")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    let input_path = matches.value_of("input").unwrap();
    let input_data = fs::read_to_string(input_path).expect("Unable to read input file");

    let output_path = matches.value_of("output").unwrap();
    let output_data = fs::read_to_string(output_path).expect("Unable to read output file");

    let input_parsing = parse_input(&input_data);
    let output_parsing = parse_output(&output_data);
    match (&input_parsing, &output_parsing) {
        (Ok((_, input)), Ok((_, output))) => match validate_output(&input, &output) {
            Err(errors) => {
                for error in errors {
                    eprintln!("error: {}", error)
                }
            }
            Ok(()) => {
                let score = score(&input, &output);
                println!("score: {}", score)
            }
        },
        _ => {
            if let Err(nom::Err::Error(e)) = input_parsing {
                eprintln!("{}", convert_error(input_data.as_str(), e))
            }
            if let Err(nom::Err::Error(e)) = output_parsing {
                eprintln!("{}", convert_error(input_data.as_str(), e))
            }
        }
    }
    Ok(())
}
