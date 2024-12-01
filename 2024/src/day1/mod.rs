use aoc_runner_derive::{aoc, aoc_generator};

pub fn parse_input_day1(input: &str) -> Vec<Vec<i32>> {
    let mut columns = Vec::new();
    for line in input.lines() {
        for (i, x) in line.split_whitespace().enumerate() {
            let num = x.parse::<i32>().unwrap();
            if columns.len() <= i {
                columns.push(Vec::new());
            }
            columns[i].push(num);
        }
    }
    columns 
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let input = parse_input_day1(input);
    let sorted_columns = sort_columns(&input);
    let result = substract_columns(&sorted_columns);

    result.iter().sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let input = parse_input_day1(input);
    let sorted_columns = sort_columns(&input);

    calculate_similarity(&sorted_columns)
}

fn sort_columns(columns: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut sorted_columns = columns.clone();
    for column in sorted_columns.iter_mut() {
        column.sort();
    }

    sorted_columns
}

fn substract_columns(columns: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..columns[0].len() {
        let diff = columns[0][i] - columns[1][i];
        result.push(diff.abs());
    }

    result
}

fn calculate_similarity(sorted_columns: &Vec<Vec<i32>>) -> i32 {
    let freq_col0 = count_frequencies(&sorted_columns[0]);
    let freq_col1 = count_frequencies(&sorted_columns[1]);

    let mut i = 0;
    let mut j = 0;
    let mut result = 0;
    
    while i < freq_col0.len() && j < freq_col1.len() {
        if freq_col0[i].0 == freq_col1[j].0 {
            let value = freq_col0[i].0;
            let count0 = freq_col0[i].1;
            let count1 = freq_col1[j].1;
            result += value * (count0 as i32) * (count1 as i32);
            i += 1;
            j += 1;
        } else if freq_col0[i].0 < freq_col1[j].0 {
            i += 1;
        } else {
            j += 1;
        }
    }

    result
}

fn count_frequencies(sorted_list: &[i32]) -> Vec<(i32, usize)> {
    let mut frequencies = Vec::new();
    let mut current_value = sorted_list[0];
    let mut count = 1;

    for &value in &sorted_list[1..] {
        if value == current_value {
            count += 1;
        } else {
            frequencies.push((current_value, count));
            current_value = value;
            count = 1;
        }
    }
    frequencies.push((current_value, count));

    frequencies
}