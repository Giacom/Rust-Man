use sfml::graphics::{RenderTarget, RectangleShape};
use sfml::traits::Drawable;

#[derive(Clone)]
pub enum LevelType {
    SPACE,
    WALL
}

pub struct LevelObject<'s> {
    pub sprite: RectangleShape<'s>,
    pub level_type: LevelType
}

impl <'s> LevelObject<'s> {
    pub fn new(level_type: LevelType) -> LevelObject<'s> {
        LevelObject {
            sprite: RectangleShape::new().unwrap(),
            level_type: level_type
        }
    }
}

impl<'s> Drawable for LevelObject<'s> {
    fn draw<RT: RenderTarget>(&self, target: &mut RT) {
        self.sprite.draw(target);
    }
}