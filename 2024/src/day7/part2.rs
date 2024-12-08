use crate::day7::models::Equation;

pub fn find_equation_part2(equations: &Vec<Equation>) -> u64 {
    let mut count:u64 = 0;
    for i in 0..equations.len() {
        let mut result = equations[i].inputs[0];
        for j in 1..equations[i].inputs.len() {
            result *= equations[i].inputs[j];
            if result > equations[i].result {
                break;
            }
        }
        if result == equations[i].result {
            count += result;
            continue;
        }

        if bruteforce_part2(&equations[i].inputs, equations[i].result, 0) {
            count += equations[i].result;
        }

    }
    count
}

fn bruteforce_part2(inputs: &Vec<u64>, result: u64, r: u64) -> bool {
    if inputs.len() == 0 {
        return r == result;
    }

    if r > result {
        return false;
    }
    let mut new_inputs = inputs.clone();
    let current = new_inputs.remove(0);

    bruteforce_part2(&new_inputs, result,r + current) || bruteforce_part2(&new_inputs, result, r * current) || bruteforce_part2(&new_inputs, result, concat(r, current))
}
fn concat(a: u64, b: u64) -> u64 {
    (a.to_string() + &b.to_string())
        .parse::<u64>()
        .unwrap()
}