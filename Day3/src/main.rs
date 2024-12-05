type Input = (u32, u32);

pub fn parse(input: &str) -> Input {
    let mem = input.as_bytes();
    let mut index = 0;
    let mut enabled = true;
    let mut part_one = 0;
    let mut part_two = 0;

    while index < mem.len() {
        if mem[index] != b'm' && mem[index] != b'd' {
            index += 1;
            continue;
        }

        if mem[index..].starts_with(b"mul(") {
            index += 4;
        } else if mem[index..].starts_with(b"do()") {
            index += 4;
            enabled = true;
            continue;
        } else if mem[index..].starts_with(b"don't()") {
            index += 7;
            enabled = false;
            continue;
        } else {
            index += 1;
            continue;
        }

        let mut first = 0;

        while mem[index].is_ascii_digit() {
            first = 10 * first + (mem[index] - b'0') as u32;
            index += 1;
        }

        if mem[index] != b',' {
            continue;
        }
        index += 1;

        let mut second = 0;

        while mem[index].is_ascii_digit() {
            second = 10 * second + (mem[index] - b'0') as u32;
            index += 1;
        }

        if mem[index] != b')' {
            continue;
        }
        index += 1;

        let sum = first * second;
        part_one += sum;
        if enabled {
            part_two += sum;
        }
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}
