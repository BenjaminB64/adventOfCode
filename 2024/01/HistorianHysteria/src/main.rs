fn main() {
    // read input example file
    let input = include_str!("../inputs/input");
    let input = get_lists(input);
    
    // sort columns
    let sorted_columns = sort_columns(input);
    println!("{:?}", sorted_columns);
    
    // substract columns
    let result = substract_columns(sorted_columns);
    println!("{:?}", result);
    
    // sum the result
    let sum: i32 = result.iter().sum();
    println!("{:?}", sum);
    
}

fn get_lists(input: &str) -> Vec<Vec<i32>> {
    let mut columns: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if columns.len() < numbers.len() {
            columns.resize_with(numbers.len(), Vec::new);
        }
        
        for (i, &num) in numbers.iter().enumerate() {
            columns[i].push(num);
        }
    }

    columns
}

fn sort_columns(columns: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut sorted_columns = columns.clone();
    for column in sorted_columns.iter_mut() {
        column.sort();
    }

    sorted_columns
}

fn substract_columns(columns: Vec<Vec<i32>>) -> Vec<i32> {
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