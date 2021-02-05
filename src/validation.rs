use crate::parser::{Input, Output, N};

use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub(crate) enum InvalidOutputError {
    #[error("found `{0}` waves but expected `{1}`")]
    WaveNumber(N, N),
    #[error("wave: `{0}` found `{1}` towers but expected `{2}`")]
    WaveTowers(usize, N, N),
    #[error("total towers cost `{0}` > budget: `{1}`")]
    Budget(N, N),
}

#[derive(Error, Debug, Eq, PartialEq)]
pub(crate) enum InvalidInputError {
    #[error("found `{0}` bonus but expected `{1}`")]
    Bonus(N, N),
    #[error("found `{0}` cost but expected `{1}`")]
    Cost(N, N),
    #[error("found `{0}` waves but expected `{1}`")]
    Waves(N, N),
    #[error("wave: `{0}` found `{1}` units but expected `{2}`")]
    WaveUnits(usize, N, N),
    #[error("found `{0}` hits but expected `{1}`")]
    Hits(N, N),
    #[error("tower: `{0}` found `{1}` units but expected `{2}`")]
    TowerHits(usize, N, N),
}

pub(crate) fn validate_output(
    input: &Input,
    output: &Output,
) -> Result<(), Vec<InvalidOutputError>> {
    let mut errors = vec![];
    if output.waves.len() != input.header.waves {
        errors.push(InvalidOutputError::WaveNumber(
            output.waves.len(),
            input.header.waves,
        ))
    }
    for (line, wave_towers) in output.waves.iter().enumerate() {
        if wave_towers.len() != input.header.towers {
            errors.push(InvalidOutputError::WaveTowers(
                line,
                wave_towers.len(),
                input.header.towers,
            ))
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
        errors.push(InvalidOutputError::Budget(total_cost, input.header.budget))
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub(crate) fn validate_input(input: &Input) -> Result<(), Vec<InvalidInputError>> {
    let mut errors = vec![];
    if input.body.bonus.len() != input.header.waves {
        errors.push(InvalidInputError::Bonus(
            input.body.bonus.len(),
            input.header.waves,
        ))
    }
    if input.body.costs.len() != input.header.towers {
        errors.push(InvalidInputError::Cost(
            input.body.costs.len(),
            input.header.towers,
        ))
    }
    if input.body.waves.len() != input.header.waves {
        errors.push(InvalidInputError::Waves(
            input.body.waves.len(),
            input.header.waves,
        ))
    }
    for (wave, wave_units) in input.body.waves.iter().enumerate() {
        if wave_units.len() != input.header.units {
            errors.push(InvalidInputError::WaveUnits(
                wave,
                wave_units.len(),
                input.header.units,
            ))
        }
    }
    if input.body.hits.len() != input.header.towers {
        errors.push(InvalidInputError::Hits(
            input.body.hits.len(),
            input.header.towers,
        ))
    }
    for (tower, tower_hits) in input.body.hits.iter().enumerate() {
        if tower_hits.len() != input.header.units {
            errors.push(InvalidInputError::TowerHits(
                tower,
                tower_hits.len(),
                input.header.units,
            ))
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{Body, Header, Input, Output};
    use crate::validation::validate_output;

    #[test]
    fn test_validate_output_valid() {
        let input = Input {
            header: Header {
                units: 1,
                towers: 2,
                waves: 1,
                budget: 2,
            },
            body: Body {
                hits: vec![vec![1], vec![1]],
                waves: vec![vec![1]],
                bonus: vec![3],
                costs: vec![1, 1],
            },
        };
        let output = Output {
            waves: vec![vec![1, 1]],
        };
        let valid = validate_output(&input, &output);
        assert_eq!(valid, Ok(()))
    }
}
