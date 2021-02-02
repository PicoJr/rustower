from dataclasses import dataclass
from typing import List


@dataclass
class InputHeader:
    units: int
    towers: int
    waves: int
    budget: int


@dataclass
class InputBody:
    hits: List[List[int]]
    costs: List[int]
    bonus: List[int]
    waves: List[List[int]]


@dataclass
class Input:
    header: InputHeader
    body: InputBody


@dataclass
class Output:
    towers: List[List[int]]
