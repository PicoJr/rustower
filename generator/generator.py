from random import randint
from typing import List

from data import Input, InputHeader, InputBody


def random_costs(towers: int) -> List[int]:
    return [randint(1, 10) for _tower in range(towers)]


def random_bonus(waves: int) -> List[int]:
    return [randint(1, 10) for _wave in range(waves)]


def random_hits(towers: int, units: int) -> List[List[int]]:
    return [[randint(1, 10) for _unit in range(units)] for _tower in range(towers)]


def random_waves(waves: int, units: int) -> List[List[int]]:
    return [[randint(1, 10) for _unit in range(units)] for _wave in range(waves)]


def create_input(units: int, towers: int, waves: int, budget: int) -> Input:
    header = InputHeader(units, towers, waves, budget)
    rdm_costs = random_costs(header.towers)
    rdm_bonus = random_bonus(header.waves)
    rdm_waves = random_waves(header.waves, header.units)
    rdm_hits = random_hits(header.towers, header.units)
    body = InputBody(rdm_hits, rdm_costs, rdm_bonus, rdm_waves)
    return Input(header, body)


if __name__ == "__main__":
    rdm_input = create_input(2, 2, 2, 10)
    print(rdm_input)
