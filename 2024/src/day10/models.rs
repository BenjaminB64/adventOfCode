use std::collections::HashSet;

pub struct Map {
    pub height: i32,
    pub width: i32,
    pub rows: Vec<Vec<u8>>,
}

impl Map {
    pub fn get(&self, x: i32, y: i32) -> u8 {
        self.rows[y as usize][x as usize]
    }
    #[allow(dead_code)]
    pub fn to_string(&self, trailheads: HashSet<(u32, u32)>) -> String {
        let mut s = String::new();
        
        for i in 0..self.height {
            for j in 0..self.width {
                if trailheads.contains(&(j as u32, i as u32)) {
                    s.push(self.get(j, i) as char);
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        s
    }
}