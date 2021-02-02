from data import Input, Output
from parser import parse, output_as_lines, dump_output


def evaluate(tower: int, wave: int, input_data: Input):
    assert 0 <= tower < input_data.header.towers
    tower_cost = input_data.body.costs[tower]
    assert tower_cost > 0
    hits = sum(input_data.body.hits[tower])
    return hits / tower_cost


def solve(input_data: Input) -> Output:
    towers = []
    for _wave in range(input_data.header.waves):

        def heuristic(tower):
            return evaluate(tower, _wave, input_data)

        wave = [0] * input_data.header.towers
        best_tower = max(range(input_data.header.towers), key=heuristic)
        n_towers = int(
            input_data.header.budget
            / (input_data.header.waves * input_data.body.costs[best_tower])
        )
        wave[best_tower] = n_towers
        towers.append(wave)
    return Output(towers)


if __name__ == "__main__":
    input_data = parse("../example/input_example.txt")
    print(input_data)
    output_data = solve(input_data)
    print(output_data)
    print(output_as_lines(output_data))
    dump_output("/tmp/out.txt", output_data)
