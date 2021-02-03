from dataclasses import dataclass
from random import randint, uniform, shuffle
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
    units_range: (int, int)
    hits_range: (int, int)


def sparse_random_list(size: int, value_range: (int, int)) -> List[int]:
    non_null = randint(1, size)
    wave = [randint(*value_range) for _v in range(non_null)] + [0] * (size - non_null)
    shuffle(wave)
    return wave


def random_budget(estimated_budget: int, budget_range: (float, float)) -> int:
    return int(estimated_budget * uniform(*budget_range))


def random_costs(towers: int) -> List[int]:
    return [randint(1, 10) for _tower in range(towers)]


def random_wave_bonus(wave_units: int, bonus_range: (float, float)) -> int:
    return int(wave_units * uniform(*bonus_range))


def random_waves_bonus(
    waves: List[List[int]], bonus_range: (float, float)
) -> List[int]:
    return [random_wave_bonus(sum(wave), bonus_range) for wave in waves]


def random_hits(towers: int, hits_range: (int, int)) -> List[List[int]]:
    return [sparse_random_list(towers, hits_range) for _tower in range(towers)]


def random_wave(units: int, units_range: (int, int)) -> List[int]:
    return sparse_random_list(units, units_range)


def random_waves(waves: int, units: int, units_range: (int, int)) -> List[List[int]]:
    return [random_wave(units, units_range) for _wave in range(waves)]


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
    print(f"average cost: {avg_cost}, average_hits: {avg_hits}, units: {units}")
    return int(units * (avg_cost / avg_hits))


def create_input(random_parameters: RandomParameters) -> Input:
    rdm_waves = random_waves(
        random_parameters.waves, random_parameters.units, random_parameters.units_range
    )
    rdm_costs = random_costs(random_parameters.towers)
    rdm_bonus = random_waves_bonus(rdm_waves, random_parameters.bonus_range)
    rdm_hits = random_hits(random_parameters.towers, random_parameters.hits_range)

    estimated_budget = estimate_budget(rdm_hits, rdm_costs, rdm_waves)
    print(f"estimated budget: {estimated_budget}")
    rdm_budget = random_budget(estimated_budget, random_parameters.budget_range)
    print(f"random budget: {rdm_budget}")

    header = InputHeader(
        random_parameters.units,
        random_parameters.towers,
        random_parameters.waves,
        rdm_budget,
    )
    body = InputBody(rdm_hits, rdm_costs, rdm_bonus, rdm_waves)
    assert header.towers == len(body.hits)
    assert header.waves == len(body.waves)
    assert header.towers == len(body.costs)
    return Input(header, body)


if __name__ == "__main__":
    # small size, easy to parse and reason about
    input_1 = RandomParameters(2, 2, 2, (1, 2), (1, 2), (1, 5), (1, 5))
    input_2 = RandomParameters(20, 20, 20, (1, 2), (1, 2), (1, 10), (1, 5))
    # big bonus, really worth it
    input_3 = RandomParameters(30, 30, 30, (1, 2), (2, 4), (1, 10), (1, 10))
    # many towers, should break unoptimized tower selection heuristics
    input_4 = RandomParameters(10, 1000, 100, (1, 2), (1, 2), (1, 20), (1, 10))
    # many everything, small budget, small bonus, bonus are not worth it
    input_5 = RandomParameters(500, 500, 500, (0.5, 1), (0.2, 0.5), (1, 10), (1, 10))
    for i, parameter in enumerate([input_1, input_2, input_3, input_4, input_5]):
        rdm_input = create_input(parameter)
        print(rdm_input.header)
        dump_input(f"input_{i}.txt", rdm_input)
