use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    get_lists(input)
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<Vec<i32>>) -> i32 {
    let sorted_columns = sort_columns(input);
    let result = substract_columns(&sorted_columns);

    result.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<Vec<i32>>) -> i32 {
    let sorted_columns = sort_columns(input);

    calculate_similarity(&sorted_columns)
}
fn get_lists(input: &str) -> Vec<Vec<i32>> {
    let mut columns = vec![Vec::new(), Vec::new()];
    for line in input.lines() {
        let numbers: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        for (i, &num) in numbers.iter().enumerate() {
            columns[i].push(num);
        }
    }
    columns
}

fn sort_columns(columns: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut sorted_columns = columns.clone();
    for column in sorted_columns.iter_mut() {
        column.sort();
    }

    sorted_columns
}

fn substract_columns(columns: &Vec<Vec<i32>>) -> Vec<i32> {
    // for each row, get the difference between all the columns
    let mut result = Vec::new();
    for i in 0..columns[0].len() {
        let mut diff = columns[0][i];
        for j in 1..columns.len() {
            diff -= columns[j][i];
        }
        result.push(diff.abs());
    }

    result
}

fn calculate_similarity(sorted_columns: &Vec<Vec<i32>>) -> i32 {
    let mut index_right = 0;

    let mut last_look_for = 0;
    let mut last_nb_similar_right;
    let mut last_result = 0;

    let mut result = 0;

    for i in 0..sorted_columns[0].len() {
        if last_look_for == sorted_columns[0][i] {
            result += last_result;
            continue;
        }
        last_look_for = sorted_columns[0][i];
        // look for the same element in the right column and count the number of similar elements
        (last_nb_similar_right, index_right) = look_for_element(&sorted_columns[1], last_look_for, index_right);

        last_result = last_look_for * last_nb_similar_right;
        result += last_result;
    }

    result
}

fn look_for_element(sorted_column: &Vec<i32>, look_for: i32, index: i32) -> (i32, i32) {
    let mut nb_similar = 0;
    let mut index_end = 0;

    for j in index as usize..sorted_column.len() {
        if sorted_column[j] == look_for {
            nb_similar += 1;
            index_end = j;
            continue;
        }
        if sorted_column[j] > look_for {
            break;
        }
    }

    (nb_similar, index_end as i32)
}