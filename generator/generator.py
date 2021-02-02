from dataclasses import dataclass
from random import randint, uniform
from typing import List

from data import Input, InputHeader, InputBody
from parser import dump_input

SMALL_BONUS_MULTIPLIER_RANGE = (1, 2)
BIG_BONUS_MULTIPLIER_RANGE = (2, 4)


@dataclass
class RandomParameters:
    units: int
    towers: int
    waves: int
    budget_range: (float, float)
    bonus_range: (float, float)


def random_budget(estimated_budget: int, budget_range: (float, float)) -> int:
    return int(estimated_budget * uniform(*budget_range))


def random_costs(towers: int) -> List[int]:
    return [randint(1, 10) for _tower in range(towers)]


def random_bonus_wave(wave_units: int, bonus_range: (float, float)) -> int:
    return int(wave_units * uniform(*bonus_range))


def random_bonus_waves(
    waves: List[List[int]], bonus_range: (float, float)
) -> List[int]:
    return [random_bonus_wave(sum(wave), bonus_range) for wave in waves]


def random_hits(towers: int, units: int) -> List[List[int]]:
    return [[randint(1, 10) for _unit in range(units)] for _tower in range(towers)]


def random_waves(waves: int, units: int) -> List[List[int]]:
    return [[randint(1, 10) for _unit in range(units)] for _wave in range(waves)]


def average_cost(costs: List[int]) -> float:
    # assume costs is not empty
    return sum(costs) / len(costs)


def average_hits(hits: List[List[int]]) -> float:
    # assume hits and tower hits are not empty
    # compute averages of averages
    return sum([sum(tower_hits) / len(tower_hits) for tower_hits in hits]) / len(hits)


def total_units(waves: List[List[int]]) -> int:
    return sum(sum(units) for units in waves)


def estimate_budget(
    hits: List[List[int]], costs: List[int], waves: List[List[int]]
) -> int:
    avg_cost = average_cost(costs)
    avg_hits = average_hits(hits)
    units = total_units(waves)
    return int(units * (avg_hits / avg_cost))


def create_input(random_parameters: RandomParameters) -> Input:

    rdm_waves = random_waves(random_parameters.waves, random_parameters.units)
    rdm_costs = random_costs(random_parameters.towers)
    rdm_bonus = random_bonus_waves(rdm_waves, random_parameters.bonus_range)
    rdm_hits = random_hits(random_parameters.towers, random_parameters.units)

    estimated_budget = estimate_budget(rdm_hits, rdm_costs, rdm_waves)
    rdm_budget = random_budget(estimated_budget, random_parameters.budget_range)

    header = InputHeader(
        random_parameters.units,
        random_parameters.towers,
        random_parameters.waves,
        rdm_budget,
    )
    body = InputBody(rdm_hits, rdm_costs, rdm_bonus, rdm_waves)
    return Input(header, body)


if __name__ == "__main__":
    # small size, easy to parse and reason about
    input_1 = RandomParameters(2, 2, 2, (1, 2), (1, 2))
    input_2 = RandomParameters(20, 20, 20, (1, 2), (1, 2))
    # big bonus, really worth it
    input_3 = RandomParameters(30, 30, 30, (1, 2), (2, 4))
    # many towers, should break unoptimized tower selection heuristics
    input_4 = RandomParameters(10, 1000, 100, (1, 2), (1, 2))
    # many everything, small budget, small bonus, bonus are not worth it
    input_5 = RandomParameters(500, 500, 500, (0.5, 1), (0.2, 0.5))
    for i, parameter in enumerate([input_1, input_2, input_3, input_4, input_5]):
        rdm_input = create_input(parameter)
        print(rdm_input.header)
        dump_input(f"input_{i}.txt", rdm_input)
