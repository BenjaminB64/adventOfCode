pub fn parse_input_day11(input: &str) -> Vec<u128> {
    let mut stones = vec![];

    let mut actual_stone: u128 = 0;
    for b in input.bytes() {
        if b == b'\n' {
            stones.push(actual_stone);
            actual_stone = 0;
            break;
        } else if b == b' ' {
            stones.push(actual_stone);
            actual_stone = 0;
        } else {
            actual_stone = actual_stone * 10 + (b - b'0') as u128;
        }
    }
    if actual_stone != 0 {
        stones.push(actual_stone);
    }
    // println!("{:?}", stones);
    stones
}

pub fn calc_stones(stones: &mut Vec<u128>, nb_blink: u16) -> u64 {
    for _ in 0..nb_blink {
        //clone_stones = stones.clone();
        let mut i_clone = 0;

        for mut i in 0..stones.len() {
            let v = &stones[i];
            if *v == 0 {
                stones[i] = 1;
                i += 1;
                continue;
            }
            if len_n(v) % 2 == 0 {
                let (first, second) = split_number_two_parts(v);
                stones[i] = first;
                stones.insert(i + 1, second);
                i += 2;
                continue;
            }
            stones[i] *= 2024;
            i_clone += 1;
        }

        println!("{:?}", stones.len());
    }

    stones.len() as u64
}

fn len_n(n: &u128) -> u16 {
    if *n == 0 {
        return 1;
    }

    (n.ilog10() + 1) as u16
}

fn split_number_two_parts(n: &u128) -> (u128, u128) {
    let len = len_n(n);
    let first = *n / 10u128.pow((len / 2) as u32);
    let second = *n % 10u128.pow((len / 2) as u32);

    (first, second)
}