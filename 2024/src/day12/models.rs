pub struct Garden {
    pub(crate) plots: Vec<Vec<Plot>>,
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
}