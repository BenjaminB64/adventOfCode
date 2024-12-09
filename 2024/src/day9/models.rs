pub struct DiskMap {
    pub blocks: Vec<Block>,
}

#[derive(Debug)]
pub struct Block {
    pub id: Option<u16>,
    pub free_space: bool,
    pub size: u8,
}

impl Clone for Block {
    fn clone(&self) -> Self {
        Block {
            id: self.id,
            free_space: self.free_space,
            size: self.size,
        }
    }
}

impl Clone for DiskMap {
    fn clone(&self) -> Self {
        DiskMap {
            blocks: self.blocks.clone(),
        }
    }
}

impl DiskMap {
    pub fn to_string(&self) -> String {
        blocks_to_string(&self.blocks)
    }
}

pub fn blocks_to_string(blocks: &Vec<Block>) -> String {
    let mut s = String::new();
    for b in blocks {
        for _ in 0..b.size {
            s.push(if b.free_space { '.' } else { b.id.unwrap_or(0).to_string().chars().next().unwrap() });
        }
    }
    s
}