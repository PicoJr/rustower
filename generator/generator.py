from dataclasses import dataclass
from random import randint, uniform, shuffle
from typing import List

from data import Input, InputHeader, InputBody
from parser import dump_input


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


def random_hits(towers: int, units: int, hits_range: (int, int)) -> List[List[int]]:
    hits = [sparse_random_list(units, hits_range) for _tower in range(towers)]
    return hits


def vulnerabilities(units: int, hits: List[List[int]]) -> List[bool]:
    vulnerable_units = [
        False
    ] * units  # True means a tower is effective against this unit
    for tower_hits in hits:
        for unit, hit in enumerate(tower_hits):
            if hit > 0:
                vulnerable_units[unit] = True
        if all(vulnerable_units):
            return vulnerable_units
    return vulnerable_units


def patch_vulnerabilities(
    towers: int, hits: List[List[int]], vulnerable_units: List[bool]
) -> List[List[int]]:
    for unit, vuln in enumerate(vulnerable_units):
        random_tower = randint(0, towers)
        if not vuln:
            hits[random_tower][unit] = 1
    return hits


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
    rdm_hits = random_hits(
        random_parameters.towers, random_parameters.units, random_parameters.hits_range
    )
    vulnerable_units = vulnerabilities(random_parameters.units, rdm_hits)
    # make sure all units can be stopped by at least one tower
    patch_vulnerabilities(random_parameters.towers, rdm_hits, vulnerable_units)
    vulnerable_units = vulnerabilities(random_parameters.units, rdm_hits)
    assert all(vulnerable_units)

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
    input_1 = RandomParameters(
        units=2,  # small
        towers=2,  # small
        waves=2,  # small
        budget_range=(1, 2),
        bonus_range=(1, 2),
        units_range=(1, 5),
        hits_range=(1, 5),
    )
    input_2 = RandomParameters(
        units=20,
        towers=20,
        waves=20,
        budget_range=(1, 2),
        bonus_range=(1, 2),
        units_range=(1, 10),
        hits_range=(1, 5),
    )
    input_3 = RandomParameters(
        units=30,
        towers=30,
        waves=30,
        budget_range=(1, 2),
        bonus_range=(2, 4),  # big bonus, really worth it
        units_range=(1, 10),
        hits_range=(1, 10),
    )
    input_4 = RandomParameters(
        units=10,
        towers=1000,  # many towers, should break unoptimized tower selection heuristics
        waves=100,
        budget_range=(1, 2),
        bonus_range=(1, 2),
        units_range=(1, 20),
        hits_range=(1, 10),
    )
    input_5 = RandomParameters(
        units=500,  # many units
        towers=500,  # many towers
        waves=500,  # many waves
        budget_range=(0.5, 1),  # small budget
        bonus_range=(0.2, 0.5),  # small bonus, not worth it
        units_range=(1, 10),
        hits_range=(1, 10),
    )
    for i, parameter in enumerate([input_1, input_2, input_3, input_4, input_5]):
        rdm_input = create_input(parameter)
        print(rdm_input.header)
        dump_input(f"../input/input_{i}.txt", rdm_input)
