# Rustower

Concept: similar to Google Hashcode qualification round.

## Problem Statement

### Input File

```
UNITS TOWERS WAVES BUDGET

# hits required from each tower for unit 0, always > 0
HIT_1 HIT_2 ... HIT_T
# hits required from each tower for unit 1, always > 0
...
# hits required from each tower for unit U, always > 0
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

### Solution/Output File

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

`T_1 / HIT_1 + T_2 / HIT_2 + ... + T_T / HIT_T`
