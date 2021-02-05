use crate::parser::{Input, Output, N};

use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub(crate) enum OutputValidationError {
    #[error("found `{0}` waves but expected `{1}`")]
    InvalidWaveNumber(N, N),
    #[error("wave: `{0}` found `{1}` towers but expected `{2}`")]
    InvalidWaveTowers(usize, N, N),
    #[error("total towers cost `{0}` > budget: `{1}`")]
    OverBudget(N, N),
}

#[derive(Error, Debug, Eq, PartialEq)]
pub(crate) enum InputValidationError {
    #[error("found `{0}` bonus but expected `{1}`")]
    InvalidBonus(N, N),
    #[error("found `{0}` cost but expected `{1}`")]
    InvalidCost(N, N),
    #[error("found `{0}` waves but expected `{1}`")]
    InvalidWaves(N, N),
    #[error("found `{0}` hits but expected `{1}`")]
    InvalidHits(N, N),
}

pub(crate) fn validate_output(
    input: &Input,
    output: &Output,
) -> Result<(), Vec<OutputValidationError>> {
    let mut errors = vec![];
    if output.waves.len() != input.header.waves {
        errors.push(OutputValidationError::InvalidWaveNumber(
            output.waves.len(),
            input.header.waves,
        ))
    }
    for (line, wave_towers) in output.waves.iter().enumerate() {
        if wave_towers.len() != input.header.towers {
            errors.push(OutputValidationError::InvalidWaveTowers(
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
        errors.push(OutputValidationError::OverBudget(
            total_cost,
            input.header.budget,
        ))
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub(crate) fn validate_input(input: &Input) -> Result<(), Vec<InputValidationError>> {
    let mut errors = vec![];
    if input.body.bonus.len() != input.header.waves {
        errors.push(InputValidationError::InvalidBonus(
            input.body.bonus.len(),
            input.header.waves,
        ))
    }
    if input.body.costs.len() != input.header.towers {
        errors.push(InputValidationError::InvalidCost(
            input.body.costs.len(),
            input.header.towers,
        ))
    }
    if input.body.waves.len() != input.header.waves {
        errors.push(InputValidationError::InvalidWaves(
            input.body.waves.len(),
            input.header.waves,
        ))
    }
    if input.body.hits.len() != input.header.towers {
        errors.push(InputValidationError::InvalidHits(
            input.body.hits.len(),
            input.header.towers,
        ))
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
