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
        if is_safe(&lines[i], None) == 0 {
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
        if is_safe(&lines[i], None) == 0 {
            nb_safe += 1;
            continue;
        }
        for j in 0..lines[i].len() {
            if is_safe(&lines[i], Some(j)) == 0 {
                nb_safe += 1;
                break;
            }
        }
    }
    nb_safe
}

pub fn is_safe(line: &[u8], index_ignore: Option<usize>) -> usize {
    let mut last_index = None;
    let mut direction = None; 
    let mut current_direction;
    
    for i in 0..line.len() {
        if let Some(index) = index_ignore {
            if i == index {
                continue;
            }
        }

        if let Some(last_i) = last_index {
            if line[i].abs_diff(line[last_i]) > 3 || line[i] == line[last_i] {
                return i;
            }

            current_direction = line[i] > line[last_i];

            if let Some(dir) = direction {
                if dir != current_direction {
                    return i;
                }
            } else {
                direction = Some(current_direction);
            }
        }

        last_index = Some(i);
    }

    0
}
