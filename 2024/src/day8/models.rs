use std::collections::HashMap;

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub points: Vec<Vec<Point>>,
    pub antennas: HashMap<char, Vec<(u32, u32)>>
}

#[derive(Debug)]
pub struct Point {
    pub ant: Option<char>,
    pub antinodes: Vec<char>
}

impl Map {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for row in &self.points {
            for point in row {
                match point.ant {
                    Some(c) => result.push(c),
                    None => if point.antinodes.len() > 0 {
                        result.push('#');
                    } else {
                        result.push('.');
                    }
                }
            }
            result.push('\n');
        }
        result
    }
}
impl Clone for Map {
    fn clone(&self) -> Self {
        let mut points: Vec<Vec<Point>> = vec![];
        for row in &self.points {
            let mut new_row: Vec<Point> = vec![];
            for point in row {
                new_row.push(Point{ant: point.ant, antinodes: point.antinodes.clone()});
            }
            points.push(new_row);
        }
        let mut antennas: HashMap<char, Vec<(u32, u32)>> = HashMap::new();
        for (k, v) in &self.antennas {
            antennas.insert(*k, v.clone());
        }
        Map {
            width: self.width,
            height: self.height,
            points,
            antennas
        }
    }
}