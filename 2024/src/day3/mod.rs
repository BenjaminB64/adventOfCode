use aoc_runner_derive::{aoc};

pub fn parse_input_day3_part1(input: &str) -> Vec<(u16, u16)> {
    // for each char, if it's m, try to catch mul(int,int)
    let mut i = 0;
    let mut mults: Vec<(u16, u16)> = Vec::new();

    let input_bytes = input.as_bytes();
    let look_for = "mul(";
    let look_for_len = look_for.len();
    let input_len = input.len();
    let mut found = false;

    let mut first: Vec<u8>;
    let mut second: Vec<u8>;

    while i < input.len() {
        first = Vec::new();
        second = Vec::new();
        if input_bytes[i] == b'm' {
            for k in 0..look_for_len {
                if i + k >= input_len || input_bytes[i + k] != look_for.as_bytes()[k] {
                    break;
                }
                if k == look_for.len() - 1 {
                    // found a mul(
                    found = true;
                    i += 4;
                }
            }
            if !found {
                i += 1;
                continue;
            }

            let mut j = i;
            while j < input_len && input_bytes[j] >= b'0' && input_bytes[j] <= b'9' {
                // set third digit first, then second, then first
                first.push(input_bytes[j] - b'0');
                j += 1;
            }
            if j < input_len && input_bytes[j] == b',' {
                j += 1;
                i = j;
                while j < input_len && input_bytes[j] >= b'0' && input_bytes[j] <= b'9' {
                    second.push(input_bytes[j] - b'0');
                    j += 1;
                }
                if j < input_len && input_bytes[j] == b')' {
                    // found a mul(int,int)
                    // do something with first and second
                    mults.push((u8_to_u16(&first), u8_to_u16(&second)));
                    i = j + 1;
                    continue;
                }
            }
        }
        i += 1;
    }
    mults
}

pub fn parse_input_day3_part2(input: &str) -> Vec<(u16, u16)> {
    // for each char, if it's m, try to catch mul(int,int)
    let mut i = 0;
    let mut mults: Vec<(u16, u16)> = Vec::new();

    let input_bytes = input.as_bytes();
    let look_for = "mul(";
    let look_for_len = look_for.len();
    let input_len = input.len();
    let mut found = false;

    let mut first: Vec<u8>;
    let mut second: Vec<u8>;

    let look_for_do = "do()";
    let look_for_dont = "don't()";

    'outer:
    while i < input.len() {
        // if we found a don't(), skip to the next do()
        if input_bytes[i] == b'd' {
            for k in 0..look_for_dont.len() {
                if i + k >= input_len || input_bytes[i + k] != look_for_dont.as_bytes()[k] {
                    break;
                }
                if k == look_for_dont.len() - 1 {
                    i = i + k + 1;
                    while i < input_len {
                        for k in 0..look_for_do.len() {
                            if i + k >= input_len || input_bytes[i + k] != look_for_do.as_bytes()[k] {
                                break;
                            }
                            if k == look_for_do.len() - 1 {
                                continue 'outer;
                            }
                        }
                        i += 1;
                    }
                }
            }
        }
        first = Vec::new();
        second = Vec::new();
        if input_bytes[i] == b'm' {
            for k in 0..look_for_len {
                if i + k >= input_len || input_bytes[i + k] != look_for.as_bytes()[k] {
                    break;
                }
                if k == look_for.len() - 1 {
                    // found a mul(
                    found = true;
                    i += 4;
                }
            }
            if !found {
                i += 1;
                continue;
            }
            // check if it's a mul(int,int)
            let mut j = i;
            while j < input_len && input_bytes[j] >= b'0' && input_bytes[j] <= b'9' {
                // set third digit first, then second, then first
                first.push(input_bytes[j] - b'0');
                j += 1;
            }
            if j < input_len && input_bytes[j] == b',' {
                j += 1;
                i = j;
                while j < input_len && input_bytes[j] >= b'0' && input_bytes[j] <= b'9' {
                    second.push(input_bytes[j] - b'0');
                    j += 1;
                }
                if j < input_len && input_bytes[j] == b')' {
                    // found a mul(int,int)
                    // do something with first and second
                    mults.push((u8_to_u16(&first), u8_to_u16(&second)));
                    i = j + 1;
                    continue;
                }
            }
        }
        i += 1;
    }
    mults
}

fn u8_to_u16(input: &Vec<u8>) -> u16 {
    let mut result: u16 = 0;
    for i in 0..input.len() {
        result = result + (input[i] as u16) * 10u16.pow((input.len() - i - 1) as u32);
    }
    result
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let lines = parse_input_day3_part1(input);
    let mut sum: u64 = 0;
    for line in lines {
        sum += line.0 as u64 * line.1 as u64;
    }

    sum as i32
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let lines = parse_input_day3_part2(input);
    let mut sum: u64 = 0;
    for line in lines {
        sum += line.0 as u64 * line.1 as u64;
    }

    sum as i32
}

