use crate::parser::{Input, Output, N};

pub(crate) fn score(input: &Input, output: &Output) -> usize {
    output
        .waves
        .iter()
        .zip(input.body.waves.iter())
        .enumerate()
        .map(|(wave_index, (wave_towers, wave_units))| {
            score_wave(input, wave_index, wave_units, wave_towers)
        })
        .sum()
}

fn score_wave(input: &Input, wave_index: N, wave_units: &[N], wave_towers: &[N]) -> usize {
    let stopped: Vec<N> = (0..input.header.units)
        .map(|unit| {
            wave_towers
                .iter()
                .enumerate()
                .map(|(tower, quantity)| units_stopped(input, unit, tower, *quantity))
                .sum()
        })
        .collect();
    let award_bonus = stopped
        .iter()
        .zip(wave_units)
        .all(|(stopped, units)| stopped >= units);
    let raw_score = stopped
        .iter()
        .zip(wave_units)
        .map(|(stopped, units)| std::cmp::min(stopped, units))
        .sum();
    if award_bonus {
        let bonus = input.body.bonus.get(wave_index).unwrap_or(&0);
        raw_score + bonus
    } else {
        raw_score
    }
}

fn units_stopped(input: &Input, unit: N, tower: N, quantity: N) -> usize {
    if let Some(hit) = input.body.hits.get(tower).and_then(|hits| hits.get(unit)) {
        quantity * hit
    } else {
        0
    }
}
