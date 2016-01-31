use sfml::graphics::{RenderTarget, RectangleShape, Color};
use sfml::traits::Drawable;

pub struct Player<'a> {
    pub shape: RectangleShape<'a>
}

impl<'a> Player<'a> {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Player<'a> {
        let mut player = Player {
            shape: RectangleShape::new().unwrap()
        };

        player.shape.set_size2f(w, h);
        player.shape.set_position2f(x, y);
        player.shape.set_fill_color(&Color::white());
        return player;
    }
}

impl<'a> Drawable for Player<'a> {
    fn draw<RT: RenderTarget>(&self, target: &mut RT) {
        self.shape.draw(target);
    }
}
