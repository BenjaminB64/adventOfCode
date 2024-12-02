use aoc_runner_derive::{aoc};

pub fn parse_input_day2(input: &str) -> Vec<Vec<u8>> {
    let mut lines = Vec::new();
    for line in input.lines() {
        let l = line.split_whitespace().map(|x| x.parse::<u8>().unwrap()).collect();
        lines.push(l);
    }
    lines
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u16 {
    let lines = parse_input_day2(input);
    let mut nb_safe = 0;
    for i in 0..lines.len() {
        if is_safe(&lines[i]) == 0 {
            nb_safe += 1;
        }
    }
    nb_safe
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u16 {
    let lines = parse_input_day2(input);
    let mut nb_safe = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if is_safe(&try_without_number(&lines[i], j)) == 0 {
                nb_safe += 1;
                break;
            }
        }
    }
    nb_safe
}

pub fn try_without_number(line: &Vec<u8>, index: usize) -> Vec<u8> {
    let mut new_line = Vec::new();
    for i in 0..line.len() {
        if i != index {
            new_line.push(line[i]);
        }
    }
    new_line
}

pub fn is_safe(line: &Vec<u8>) -> usize {
    let increase_or_decrease = get_increase_or_decrease(line[1], line[0]);
    for i in 1..line.len() {
        if line[i].abs_diff(line[i - 1]) > 3 || line[i] == line[i - 1] {
            return i;
        }
        if increase_or_decrease != get_increase_or_decrease(line[i], line[i - 1]) {
            return i;
        }
    }
    0
}

pub fn get_increase_or_decrease(n1: u8, n2: u8) -> bool {
    n2 > n1
}