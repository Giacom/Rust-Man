#[derive(Clone)]
pub enum LevelObject {
    SPACE,
    WALL
}

pub struct Level {
    width: usize,
    height: usize,
    map: Vec<Vec<LevelObject>>
}

impl Level {
    pub fn new(width: u32, height: u32) -> Level {
        Level {
            width: width as usize,
            height: height as usize,
            map: vec![vec![LevelObject::SPACE; height as usize]; width as usize]
        }
    }
    
    pub fn new_with_map(map: Vec<Vec<LevelObject>>) -> Level {
        Level {
            width: map.len(),
            height: map[0].len(),
            map: map
        }
    }
}