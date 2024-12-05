use aoc_runner_derive::aoc;

pub fn parse_input_day4_part1(input: &str) -> u32 {
    let look_for = "XMAS";
    let look_for_reverse = "SAMX";
    let look_for_bytes = look_for.as_bytes();
    let look_for_reverse_bytes = look_for_reverse.as_bytes();

    let look_for_len = look_for.len() as u8;
    let line_len: u16 = get_line_length(input);
    let mut nb_found = 0;
    let input_as_bytes = input.as_bytes();
    let input_len: u32 = input.len() as u32;

    let nb_lines = line_len;
    let nb_lines_limit = nb_lines - 3;
    let k_limit = line_len - 3;
    // show all lines
    for i in 0..nb_lines {
        for k in 0..line_len {
            // println!("x: {}, y: {}", k, i);
            let c = get_char_x_y_from_string(input_as_bytes, k, i as u16, line_len);
            if c != look_for_bytes[0] && c != look_for_reverse_bytes[0] {
                continue;
            }
            if k < k_limit {
                let right = get_string_right(input_as_bytes, k, i as u16, line_len, look_for_len);
                if check_string(&right, look_for_bytes, look_for_reverse_bytes) {
                    // println!("Found right at x: {}, y: {}", k, i);
                    nb_found += 1;
                }
            }

            if i < nb_lines_limit {
                let bottom = get_string_bottom(input_as_bytes, k, i as u16, line_len, look_for_len);
                if check_string(&bottom, look_for_bytes, look_for_reverse_bytes) {
                    // println!("Found bottom at x: {}, y: {}", k, i);
                    nb_found += 1;
                }
                if k < k_limit {
                    let right_bottom = get_string_right_bottom(input_as_bytes, k, i as u16, line_len, look_for_len);
                    if check_string(&right_bottom, look_for_bytes, look_for_reverse_bytes) {
                        // println!("Found right bottom at x: {}, y: {}", k, i);
                        nb_found += 1;
                    }
                }
            }

            if k >= 3 && i < nb_lines_limit {
                let left_bottom = get_string_left_bottom(input_as_bytes, k, i as u16, line_len, look_for_len);
                if check_string(&left_bottom, look_for_bytes, look_for_reverse_bytes) {
                    // println!("Found left bottom at x: {}, y: {}", k, i);
                    nb_found += 1;
                }
            }
        }
    }
    nb_found
}
pub fn get_line_length(input: &str) -> u16 {
    let mut i = 0;
    let input_as_bytes = input.as_bytes();
    let input_len: u32 = input.len() as u32;
    while i < input_len {
        i += 1;
        if input_as_bytes[i as usize] == '\n' as u8 {
            return i as u16;
        }
    }
    0
}

pub fn parse_input_day4_part2(input: &str) -> u32 {
    let look_for = "MAS";
    let look_for_reverse = "SAM";
    let look_for_bytes = look_for.as_bytes();
    let look_for_reverse_bytes = look_for_reverse.as_bytes();

    let look_for_len = look_for.len() as u8;
    let line_len: u16 = get_line_length(input);
    let mut nb_found = 0;
    let input_as_bytes = input.as_bytes();
    let input_len: u32 = input.len() as u32;
    let nb_lines = input_len / line_len as u32;
    let nb_lines_limit = nb_lines - 1;
    let k_limit = line_len - 1;

    // show all lines
    for i in 0..nb_lines {
        for k in 0..line_len {
            let c = get_char_x_y_from_string(input_as_bytes, k, i as u16, line_len);
            if c != 'A' as u8 {
                continue;
            }
            if i < nb_lines_limit && k < k_limit && i >= 1 && k >= 1 {
                let right_bottom = get_string_right_bottom(input_as_bytes, (k - 1) as u16, (i - 1) as u16, line_len, look_for_len);
                if check_string(&right_bottom, look_for_bytes, look_for_reverse_bytes) {
                    let left_bottom = get_string_left_bottom(input_as_bytes, (k + 1) as u16, (i - 1) as u16, line_len, look_for_len);
                    if check_string(&left_bottom, look_for_bytes, look_for_reverse_bytes) {
                        nb_found += 1;
                    }
                }
            }
        }
    }
    nb_found
}

pub fn check_string(s: &[u8], look_for: &[u8], look_for_reverse: &[u8]) -> bool {
    s == look_for || s == look_for_reverse
}

pub fn get_string_right(s: &[u8], x: u16, y: u16, line_len: u16, len: u8) -> &[u8] {
    let start = x_y_to_index(x, y, line_len);
    let end = start + len as usize;
    &s[start..end]
}

pub fn get_string_bottom(s: &[u8], x: u16, y: u16, line_len: u16, len: u8) -> Vec<u8> {
    let mut ret = Vec::with_capacity(len as usize);
    for i in 0..len as u16 {
        ret.push(get_char_x_y_from_string(s, x, y + i, line_len));
    }
    ret
}

pub fn get_string_left_bottom(s: &[u8], x: u16, y: u16, line_len: u16, len: u8) -> Vec<u8> {
    let mut ret = Vec::with_capacity(len as usize);
    for i in 0..len as u16 {
        ret.push(get_char_x_y_from_string(s, x - i, y + i, line_len));
    }
    ret
}

pub fn get_string_right_bottom(s: &[u8], x: u16, y: u16, line_len: u16, len: u8) -> Vec<u8> {
    let mut ret = Vec::with_capacity(len as usize);
    for i in 0..len as u16 {
        ret.push(get_char_x_y_from_string(s, x + i, y + i, line_len));
    }
    ret
}

pub fn get_char_x_y_from_string(s: &[u8], x: u16, y: u16, line_len: u16) -> u8 {
    s[x_y_to_index(x, y, line_len)]
}

pub fn x_y_to_index(x: u16, y: u16, line_len: u16) -> usize {
    y as usize * (line_len as usize + 1) + x as usize
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    parse_input_day4_part1(input)
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    parse_input_day4_part2(input)
}