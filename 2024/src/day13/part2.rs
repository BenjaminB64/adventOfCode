use crate::day13::part1::ParsedInput;

pub fn calc_needed_tokens_part2(mut parsed_inputs: Vec<ParsedInput>) -> i64 {
    let mut sum: i64 = 0;
    for mut parsed_input in &mut parsed_inputs {

        let xa = parsed_input.button_a.0 as i64;
        let ya = parsed_input.button_a.1 as i64;
        let xb = parsed_input.button_b.0 as i64;
        let yb = parsed_input.button_b.1 as i64;
        let px = parsed_input.prize.0  as i64 + 10000000000000;
        let py = parsed_input.prize.1 as i64 + 10000000000000;
        let na: i64 = (px * yb - py * xb) / (xa * yb - ya * xb);
        let nb: i64 = (py * xa - px * ya) / (xa * yb - xb * ya);
        if na < 0 || nb < 0 {
            continue;
        }
        if na * xa + nb * xb != px || na * ya + nb * yb != py {
            continue;
        }

        sum += (na * 3 + nb);
    }

    sum
}