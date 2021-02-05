![logo](./res/logo.png)

logo built using assets provided by https://kenney.nl/assets/tower-defense

# Rustower

This repository contains:
* The problem statement of a challenge inspired by [Google Hashcode](https://hashcode.withgoogle.com/)
* Input/Output validator and score-calculator reference implementation (in [Rust](https://www.rust-lang.org/))
* [Input files](./input)
* Simple [solver](generator/solver.py) and [input parser/output dumper](generator/parser.py) (Python)
* [Code for generating input files](generator/generator.py).

**Input Files are still WIP**

## Problem Statement

Compute the best towers to build in order to defend against waves of units given a budget.

### Input File

> Note: lines starting with '#' are comments, input files do not contain comments
>
> All numbers are integers

```
UNITS TOWERS WAVES BUDGET

# how many of each unit type a single tower of type 0 can stop
HIT_1 HIT_2 ... HIT_U
# how many of each unit type a single tower of type 1 can stop
...
# how many of each unit type a single tower of type T can stop
...

# cost for each tower
COST_1 COST_2 ... COST_T

# bonuses for each wave
BONUS_1 BONUS_2 ... BONUS_W

# wave 1, number of each unit
N_1 N_2 ... N_U
# wave 2, number of each unit
...
# wave W, number of each unit
...
```

#### Constraints

* 0 < UNITS <= 10000
* 0 < TOWERS <= 10000
* 0 < WAVES <= 10000
* 0 < BUDGET <= 10000

### Solution/Output File

> Note: lines starting with '#' are comments, output files do not contain comments
>
> Output file must contain exactly `WAVES` lines.
>
> Each line must contain exactly `TOWERS` integers.

```
# wave 1 towers of each type
T_1 T_2 ... T_T
# wave 2 towers of each type
...
# wave W towers of each type
...
```

### Scoring

* 1 point for each unit stopped
* for each wave: bonus wave points if all units were stopped

### Simulation

For each type of unit, compute how many units of this type towers can stop:

* given a unit `i`
* `T` towers

The number of units of type `i` stopped is sum of `N_k * HIT_k_i` for k in `0..T-1` where

* `N_k` is the number of towers of type `k`.
* `HIT_k_i` is the number of units of type `i` a single tower of type `k` can stop.

### Example

#### Input File

```
2 2 2 12
2 1
1 2
3 4
2 1
1 1
3 3
```

> 2 UNITS, 2 TOWERS, 2 WAVES, BUDGET = 12
>
> 1 TOWER of type 0 can stop 2 units of type 0 and 1 unit of type 1
>
> 1 TOWER of type 1 can stop 1 unit of type 0 and 2 units of type 1
>
> TOWER 0 costs 3, TOWER 1 costs 4
>
> WAVE 0 bonus is 2, WAVE 1 bonus is 1
> 
> WAVE 0 consists of 1 unit of type 0 and 1 unit of type 1
>
> WAVE 1 consists of 3 unit of type 0 and 3 units of type 1

#### Output File

```
1 0
0 2
```

> WAVE 1: 1 tower of type 0 and 0 tower of type 1
>
> WAVE 2: 0 tower of type 0 and 2 towers of type 1

total cost = 1 * 3 + 2 * 4 = 11 < 12 (BUDGET)

First wave:
* units of type 0 stopped = 1 (number of towers of type 0) * 2 + 0 (number of towers of type 1) * 1 = 2
* units of type 1 stopped = 1 (number of towers of type 0) * 1 + 0 (number of towers of type 1) * 2 = 1

All units were stopped, score = 1 + 1 (units stopped) + 2 (bonus) = 4

Second wave:

* units of type 0 stopped = 2 (number of towers of type 0) * 2 + 2 (number of towers of type 1) * 1 = 2
* units of type 1 stopped = 0 (number of towers of type 0) * 1 + 2 (number of towers of type 1) * 2 = 4

score = 2 + 3 (4 units of type 1 can be stopped but second wave only has 3 units of type 1) + 0 (no bonus since only 2 units of type 0 out of 3 were stopped) = 5

total score = 4 + 5 = 9

## Validate and Compute Score

example:

```
cargo run --release -- example/input_example.txt example/output_example.txt
```

example output:

```
score: 9
```