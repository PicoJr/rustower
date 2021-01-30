mod parser;
use crate::parser::{Output, Input, N, parse_input, parse_output};

extern crate nom;
extern crate clap;
use clap::{Arg, App};
use thiserror::Error;

use std::fs;

#[derive(Error, Debug)]
enum OutputValidationError {
    #[error("found `{0}` waves but expected `{1}`")]
    InvalidWaveNumber(N, N),
    #[error("wave: `{0}` found `{1}` towers but expected `{2}`")]
    InvalidWaveTowers(usize, N, N),
    #[error("total towers cost `{0}` > budget: `{1}`")]
    OverBudget(N, N),
}

fn validate_output(input: &Input, output: &Output) -> Result<(), Vec<OutputValidationError>> {
    let mut errors = vec![];
    if output.waves.len() != input.header.waves {
        errors.push(
            OutputValidationError::InvalidWaveNumber(output.waves.len(), input.header.waves)
        )
    }
    for (line, wave_towers) in output.waves.iter().enumerate() {
        if wave_towers.len() != input.header.towers {
            errors.push(
                OutputValidationError::InvalidWaveTowers(line, wave_towers.len(), input.header.towers)
            )
        }
    }
    let mut total_cost: usize = 0;
    for wave in output.waves.iter() {
        for (tower, quantity) in wave.iter().enumerate() {
            // ignore if tower cost cannot be found => InvalidWaveTowers
            let tower_cost = input.body.costs.get(tower).unwrap_or(&0) * quantity;
            total_cost += tower_cost;
        }
    }
    if total_cost > input.header.budget {
        errors.push(
            OutputValidationError::OverBudget(total_cost, input.header.budget)
        )
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn score(input: &Input, output: &Output) -> usize {
    output.waves.iter().zip(input.body.waves.iter()).enumerate().map(|(wave_index, (wave_towers, wave_units))| score_wave(input, wave_index, wave_units, wave_towers)).sum()
}

fn score_wave(input: &Input, wave_index: N, wave_units: &[N], wave_towers: &[N]) -> usize {
    let stopped: Vec<N> = (0..input.header.units).map(
        |unit| {
            wave_towers.iter().enumerate().map(
                |(tower, quantity)| {
                    units_stopped(input, unit, tower, *quantity)
                }
            ).sum()
        }
    ).collect();
    let award_bonus = stopped.iter().zip(wave_units).all(|(stopped, units)| stopped >= units);
    let raw_score = stopped.iter().zip(wave_units).map(|(stopped, units)| {
        std::cmp::min(stopped, units)
    }).sum();
    if award_bonus {
        let bonus = input.body.bonus.get(wave_index).unwrap_or(&0);
        raw_score + bonus
    } else {
        raw_score
    }
}

fn units_stopped(input: &Input, unit: N, tower: N, quantity: N) -> usize {
    if let Some(hit) = input.body.hits.get(unit).and_then(|hits| hits.get(tower)) {
        quantity / hit
    } else {
        0
    }
}

fn main() -> anyhow::Result<()> {
    let matches = App::new("score-calculator")
        .version("1.0")
        .author("PicoJr")
        .about("Validate output and compute score")
        .arg(Arg::with_name("input")
            .value_name("INPUT")
            .help("Sets input file")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("output")
            .value_name("OUTPUT")
            .help("Sets output file")
            .takes_value(true)
            .required(true)
        )
        .get_matches();
    let input_path = matches.value_of("input").unwrap();
    let input_data = fs::read_to_string(input_path).expect("Unable to read input file");

    let output_path = matches.value_of("output").unwrap();
    let output_data = fs::read_to_string(output_path).expect("Unable to read output file");

    let input_parsing = parse_input(&input_data);
    let output_parsing = parse_output(&output_data);
    match (&input_parsing, &output_parsing) {
        (Ok((_, input)), Ok((_, output))) => {
            match validate_output(&input, &output) {
                Err(errors) => {
                    for error in errors {
                        eprintln!("error: {}", error)
                    }
                }
                Ok(()) => {
                    let score = score(&input, &output);
                    println!("score: {}", score)
                }
            }
        }
        _ => {
            eprintln!("input: {:?}", input_parsing);
            eprintln!("output: {:?}", output_parsing);
        }
    }
    Ok(())
}
