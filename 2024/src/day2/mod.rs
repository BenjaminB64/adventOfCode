use aoc_runner_derive::{aoc};

pub fn parse_input_day2(input: &str) -> Vec<Vec<i32>> {
    let mut lines = Vec::new();
    for line in input.lines() {
        let l = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        lines.push(l);
    }
    lines
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let lines = parse_input_day2(input);
    let mut nb_safe = 0;
    for i in 0..lines.len() {
        if lines[i].len() <= 1 {
            nb_safe += 1;
            continue;
        }
        let mut safe = true;

        for _ in 1..lines[i].len() {
            if (is_safe(&lines[i]) != 0) {
                safe = false;
            }
        }
        if safe {
            nb_safe += 1;
        }
    }
    nb_safe
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    let lines = parse_input_day2(input);
    let mut nb_safe = 0;
    for i in 0..lines.len() {
        if lines[i].len() <= 1 {
            nb_safe += 1;
            continue;
        }
        let is_valid = is_safe(&lines[i]);
        if is_valid == 0 {
            nb_safe += 1;
            continue;
        }
        for j in 0..lines[i].len() {
            let new_line = try_without_number(&lines[i].clone(), j);
            let new_is_valid = is_safe(&new_line);
            if new_is_valid == 0 {
                nb_safe += 1;
                break;
            }
        }
    }
    nb_safe
}

pub fn try_without_number(line: &Vec<i32>, index: usize) -> Vec<i32> {
    let mut new_line = Vec::new();
    for i in 0..line.len() {
        if i != index {
            new_line.push(line[i]);
        }
    }
    new_line
}

pub fn is_safe(line: &Vec<i32>) -> usize {
    let increase_or_decrease = get_increase_or_decrease(line[1], line[0]);
    for i in 1..line.len() {
        if (line[i] - line[i - 1]).abs() > 3 || line[i] == line[i - 1] {
            return i;
        }
        if increase_or_decrease != get_increase_or_decrease(line[i], line[i - 1]) {
            return i;
        }
    }
    0
}

pub fn get_increase_or_decrease(n1: i32, n2: i32) -> i32 {
    if n2 > n1 {
        return 1;
    }
    return -1;
}