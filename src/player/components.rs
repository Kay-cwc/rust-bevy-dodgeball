use bevy::prelude::*;

use crate::objects::components::CollisionObject;

use super::systems::{CHARACTER_WIDTH, CHARACTER_HEIGHT};

#[derive(Component)]
pub struct Character {
    pub is_head: bool,
    pub last_flip: f32,
    pub size: Vec3,
    pub invulnerable_until: f32,
}

impl Default for Character {
    fn default() -> Self {
        Character {
            is_head: true,
            last_flip: 0.,
            size: Vec3::new(CHARACTER_WIDTH, CHARACTER_HEIGHT, 1.),
            invulnerable_until: 0.,
        }
    }
}

impl CollisionObject for Character {
    fn get_dimension(&self) -> (f32, f32) {
        (self.size.x, self.size.y)
    }
}