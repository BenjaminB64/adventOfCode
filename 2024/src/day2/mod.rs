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
        if is_safe(&lines[i],0) == 0 {
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
            if is_safe(&lines[i], j) == 0 {
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

pub fn is_safe(line: &Vec<u8>, ignore_index: usize) -> usize {

    let mut increase_or_decrease= false;
    let mut known = false;
    let mut last = 0;

    println!("{:?}", line);
    for i in 1..line.len() {
        if i == 1 && ignore_index == 1 {
            continue;
        }
        if i == line.len()-1 && ignore_index == line.len()-1 {
            continue;
        }
        
        if ignore_index == 0 && i+1 != ignore_index {
            last = i-1;
            println!("Last: {}", i);
        }
        println!("Last: {}, i : {}", last, i);
        if !known {
            increase_or_decrease = get_increase_or_decrease(line[i], line[last]);
            println!("Increase or decrease: {}", increase_or_decrease);
            known = true;
        }

        if line[last].abs_diff(line[i]) > 3 || line[i] == line[last] {
            println!("UNSAFE Diff: {}", line[last].abs_diff(line[i]));
            return i;
        }
        if increase_or_decrease != get_increase_or_decrease(line[i], line[last]) {
            println!("UNSAFE Increase or decrease diff: {}", get_increase_or_decrease(line[i], line[last]));
            return i;
        }
    }
    0
}

pub fn get_increase_or_decrease(n1: u8, n2: u8) -> bool {
    n2 > n1
}