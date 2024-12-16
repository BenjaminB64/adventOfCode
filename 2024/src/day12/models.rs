pub struct Garden {
    pub(crate) plots: Vec<Vec<Plot>>,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct Plot {
    pub(crate) id: u8,
    pub counted: bool,
}

#[derive(Debug)]
pub struct Region {
    pub id: u8,
    pub area: u32,
    pub perimeter: u32,
    pub sides: u32,
    pub edges: std::collections::HashSet<Edge>,
}

impl Garden {
    pub fn get_plot(&mut self, x: u32, y: u32) -> Option<&mut Plot> {
        if x >= self.plots[0].len() as u32 || y >= self.plots.len() as u32 {
            return None;
        }

        Some(&mut self.plots[y as usize][x as usize])
    }

    pub fn is_bottom(&self, x: u32, y: u32) -> bool {
        y == self.height - 1
    }
    pub fn is_right(&self, x: u32, y: u32) -> bool {
        x == self.width - 1
    }
    pub fn is_top(&self, x: u32, y: u32) -> bool {
        y == 0
    }
    pub fn is_left(&self, x: u32, y: u32) -> bool {
        x == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge(pub (u32, u32), pub (u32, u32));

impl Edge {
    pub fn new(mut p1: (u32, u32), mut p2: (u32, u32)) -> Self {
        if p2 < p1 {
            std::mem::swap(&mut p1, &mut p2);
        }
        Edge(p1, p2)
    }
}
