use sfml::graphics::{Sprite};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum LevelType {
    SPACE,
    WALL
}

pub struct LevelObject<'s> {
    pub sprite: Sprite<'s>,
    pub level_type: LevelType
}

impl<'s> LevelObject<'s> {
    pub fn new(level_type: LevelType, sprite: Sprite<'s>) -> LevelObject<'s> {
        LevelObject {
            level_type: level_type,
            sprite: sprite
        }
    }
}