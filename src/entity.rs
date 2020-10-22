use crate::configs::{WINDOW_HEIGHT, WINDOW_WIDTH};
use tetra::graphics::{Rectangle, Texture};
use tetra::math::Vec2;

// An entity can be a ball or a player. It's defined by its appearance, position and velocity
pub struct Entity {
    pub name: String,
    pub texture: Texture,
    pub position: Vec2<f32>,
    pub velocity: Vec2<f32>,
}
impl Entity {
    pub fn new(name: String, texture: Texture, position: Vec2<f32>) -> Entity {
        Entity::with_velocity(name, texture, position, Vec2::zero())
    }
    // For ball, we need a velocity
    pub fn with_velocity(
        name: String,
        texture: Texture,
        position: Vec2<f32>,
        velocity: Vec2<f32>,
    ) -> Entity {
        Entity {
            name,
            texture,
            position,
            velocity,
        }
    }
    pub fn width(&self) -> f32 {
        self.texture.width() as f32
    }
    pub fn height(&self) -> f32 {
        self.texture.height() as f32
    }
    // Using Rectangle struct will help us to detect collision
    pub fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }
    // The center will help us to calculate the offset on the ball movement
    pub fn centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0),
        )
    }
    pub fn center(&mut self) {
        self.position = Vec2::new(
            WINDOW_WIDTH / 2.0 - self.texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - self.texture.height() as f32 / 2.0,
        );
    }
}
