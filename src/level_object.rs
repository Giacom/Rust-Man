use sfml::graphics::{FloatRect};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum LevelType {
    SPACE,
    WALL
}

pub struct LevelObject {
    pub position: FloatRect,
    pub level_type: LevelType
}

impl LevelObject {
    pub fn new(level_type: LevelType) -> LevelObject {
        LevelObject {
            position: FloatRect::new(0.0, 0.0, 0.0, 0.0),
            level_type: level_type
        }
    }
}