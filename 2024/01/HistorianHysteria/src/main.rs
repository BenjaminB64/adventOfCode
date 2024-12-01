fn main() {
    // read input example file
    let input = include_str!("../inputs/input");
    let input = get_lists(input);
    
    // sort columns
    let sorted_columns = sort_columns(input);
    // println!("{:?}", sorted_columns);
    
    // substract columns
    let result = substract_columns(&sorted_columns);
    // println!("{:?}", result);
    
    // sum the result
    let sum: i32 = result.iter().sum();
    // println!("{:?}", sum);
    
    // calculate similarity (second part)
    let similarity = calculate_similarity(&sorted_columns);
    println!("{:?}", similarity);
    
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