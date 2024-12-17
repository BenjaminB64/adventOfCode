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
}

impl Garden {
    pub fn get_plot(&mut self, x: u32, y: u32) -> Option<&mut Plot> {
        if x >= self.plots[0].len() as u32 || y >= self.plots.len() as u32 {
            return None;
        }

        Some(&mut self.plots[y as usize][x as usize])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge(pub (u32, u32), pub (u32, u32));
