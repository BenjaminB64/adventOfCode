use crate::day9::models::{Block, DiskMap};

pub fn calc_checksum_part2(map: DiskMap) -> u64 {
    let mut blocks = map.blocks.clone();
    let mut checksum: u64 = 0;

    'outer:
    for i in (0..blocks.len()).rev() {
        if blocks[i].free_space {
            continue;
        }
        for f in 0..i {
            if !blocks[f].free_space {
                continue;
            }

            if blocks[f].size == blocks[i].size {
                blocks[f].free_space = false;
                blocks[f].id = blocks[i].id;
                // --
                blocks[i].free_space = true;
                blocks[i].id = None;
                continue 'outer;
            }

            if blocks[f].size > blocks[i].size {
                let free_space = blocks[f].size - blocks[i].size;
                blocks[f].size = blocks[i].size;
                blocks[f].free_space = false;
                blocks[f].id = blocks[i].id;

                blocks[i].free_space = true;
                blocks[i].id = None;
                blocks.insert(f + 1, Block { size: free_space, free_space: true, id: None });
                continue 'outer;
            }
        }
    }

    let mut inner_index = 0;
    for i in 0..blocks.len() {
        if blocks[i].free_space {
            inner_index += blocks[i].size as u64;
            continue;
        }
        for _ in 0..blocks[i].size {
            checksum += inner_index * blocks[i].id.unwrap() as u64;
            inner_index += 1;
        }
    }
    checksum
}