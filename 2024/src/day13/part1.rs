#[derive(Debug)]
pub struct ParsedInput {
    pub(crate) tokens_needed: u32,
    pub(crate) button_a: (u32, u32),
    pub(crate) button_b: (u32, u32),
    pub(crate) prize: (u32, u32),
}

pub fn parse_input(input: &str) -> Vec<ParsedInput> {
    let lines = input.lines();

    let mut parsed_inputs = vec![];
    let mut button_a = (0, 0);
    let mut button_b = (0, 0);
    let mut prize ;
    for line in lines {
        if line.starts_with("Button A") {
            // "Button A: X+64, Y+22"
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let x: u32 = tokens[2] // "X+64,"
                .trim_start_matches("X+")
                .trim_end_matches(',')
                .parse()
                .unwrap();
            let y: u32 = tokens[3] // "Y+22"
                .trim_start_matches("Y+")
                .parse()
                .unwrap();
            button_a = (x, y);
        } else if line.starts_with("Button B") {
            // "Button A: X+64, Y+22"
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let x: u32 = tokens[2] // "X+64,"
                .trim_start_matches("X+")
                .trim_end_matches(',')
                .parse()
                .unwrap();
            let y: u32 = tokens[3] // "Y+22"
                .trim_start_matches("Y+")
                .parse()
                .unwrap();
            button_b = (x, y);
        } else if line.starts_with("Prize") {
            // "Prize: X+64, Y+22"
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let x: u32 = tokens[1] // "X+64,"
                .trim_start_matches("X=")
                .trim_end_matches(',')
                .parse()
                .unwrap();
            let y: u32 = tokens[2] // "Y+22"
                .trim_start_matches("Y=")
                .parse()
                .unwrap();
            prize = (x, y);
            parsed_inputs.push(ParsedInput {
                tokens_needed: 0,
                button_a,
                button_b,
                prize,
            });
        }
    }

    parsed_inputs
}

pub fn calc_needed_tokens(mut parsed_inputs: Vec<ParsedInput>) -> u32 {
    for mut parsed_input in &mut parsed_inputs {

        let xa = parsed_input.button_a.0 as i32;
        let ya = parsed_input.button_a.1 as i32;
        let xb = parsed_input.button_b.0 as i32;
        let yb = parsed_input.button_b.1 as i32;
        let px = parsed_input.prize.0 as i32;
        let py = parsed_input.prize.1 as i32;
        let na: i32 = (px * yb - py * xb) / (xa * yb - ya * xb);
        let nb: i32 = (py * xa - px * ya) / (xa * yb - xb * ya);
        if na < 0 || nb < 0 {
            continue;
        }
        if na * xa + nb * xb != px || na * ya + nb * yb != py {
            continue;
        }
        if na > 100 || nb > 100 {
            continue;
        }
        parsed_input.tokens_needed = (na * 3 + nb) as u32;
    }

    parsed_inputs.iter().map(|pi| pi.tokens_needed).sum()
}