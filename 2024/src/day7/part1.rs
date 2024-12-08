use crate::day7::models::Equation;

pub fn parse_input_day7(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let result = parts.next().unwrap().parse().unwrap();
            let inputs = parts
                .next()
                .unwrap()
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect();
            Equation::new(result, inputs)
        })
        .collect()
}

pub fn find_equation(equations: &Vec<Equation>) -> u64 {
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

        if bruteforce(&equations[i].inputs, equations[i].result, 0) {
            count += equations[i].result;
        }
        
    }
    count
}

fn bruteforce(inputs: &Vec<u64>, result: u64, r: u64) -> bool {
    if inputs.len() == 0 {
        return r == result;
    }

    if r > result {
        return false;
    }
    let mut new_inputs = inputs.clone();
    let current = new_inputs.remove(0);

    bruteforce(&new_inputs, result,r + current) || bruteforce(&new_inputs, result, r * current)
}