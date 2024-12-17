use std::collections::HashMap;
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
    let time_start = std::time::Instant::now();
    //println!("{:?}", stones);
    let mut cache = Cache::new();

    let imax = stones.len();
    let mut len = 0;
    for i in 0..imax {
        let v = &mut stones[i];
        len += cache.count_stones(*v, nb_blink);
    }

    println!("{} {}ms", stones.len(), time_start.elapsed().as_millis());

    len
}

struct Cache {
    split: HashMap<u128, (u128, u128)>,
    len: HashMap<u128, u16>,
    stones: HashMap<(u128, u16), u64>,
}
impl Default for Cache {
    fn default() -> Self {
        Cache {
            split: HashMap::new(),
            len: HashMap::new(),
            stones: HashMap::new(),
        }
    }
}
impl Cache {
    fn new() -> Self {
        Default::default()
    }
    fn count_stones(&mut self, stone: u128, nb_blink: u16) -> u64 {
        if let Some(v) = self.stones.get(&(stone, nb_blink)) {
            return *v;
        }
        if nb_blink == 0 {
            return 1;
        }
        let r;
        if stone == 0 {
            r = self.count_stones(1, nb_blink - 1);
        } else if self.len_n(stone) % 2 == 0 {
            let (first, second) = self.split_number_two_parts(stone);
            r = self.count_stones(first, nb_blink - 1) + self.count_stones(second, nb_blink - 1);
        } else {
            r = self.count_stones(stone * 2024, nb_blink - 1);
        }
        self.stones.insert((stone, nb_blink), r);
        r
    }
    fn len_n(&mut self, n: u128) -> u16 {
        if let Some(v) = self.len.get(&n) {
            return *v;
        }
        let v = n.to_string().len() as u16;
        self.len.insert(n, v);
        v
    }

    fn split_number_two_parts(&mut self, n: u128) -> (u128, u128) {
        if let Some(v) = self.split.get(&n) {
            return *v;
        }
        let s = n.to_string();
        let l = s.len();
        let first = s[..l / 2].parse().unwrap();
        let second = s[l / 2..].parse().unwrap();
        self.split.insert(n, (first, second));
        (first, second)
    }
}
