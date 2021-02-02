from typing import TextIO, List

from data import Input, InputHeader, InputBody, Output


def parse_int_list(line: str) -> List[int]:
    return [int(s) for s in line.rstrip().split()]


def parse_header(line: str) -> InputHeader:
    values = parse_int_list(line)
    assert len(values) == 4
    return InputHeader(*values)


def parse_file(input_file: TextIO) -> Input:
    first_line = input_file.readline()
    input_header = parse_header(first_line)
    hits = []
    for tower in range(input_header.towers):
        hits.append(parse_int_list(input_file.readline()))
    costs = parse_int_list(input_file.readline())
    bonus = parse_int_list(input_file.readline())
    waves = []
    for wave in range(input_header.waves):
        waves.append(parse_int_list(input_file.readline()))
    input_body = InputBody(hits, costs, bonus, waves)
    return Input(input_header, input_body)


def parse(input_path: str) -> Input:
    with open(input_path) as input_file:
        return parse_file(input_file)


def output_as_lines(output: Output) -> List[str]:
    return [" ".join(str(tower) for tower in towers) for towers in output.towers]


def dump_output(output_path: str, output: Output):
    with open(output_path, "w+") as output_file:
        lines = output_as_lines(output)
        for line in lines:
            output_file.write(line + "\n")


def dump_input(input_path: str, input_data: Input):
    with open(input_path, "w+") as input_file:
        h = input_data.header
        header = f"{h.units} {h.towers} {h.waves} {h.budget}\n"
        costs = " ".join(str(c) for c in input_data.body.costs) + "\n"
        bonus = " ".join(str(b) for b in input_data.body.bonus) + "\n"
        waves = [
            " ".join(str(u) for u in units) + "\n" for units in input_data.body.waves
        ]
        lines = [header, costs, bonus] + waves
        input_file.writelines(lines)


if __name__ == "__main__":
    input_data = parse("../example/input_example.txt")
    print(input_data)
