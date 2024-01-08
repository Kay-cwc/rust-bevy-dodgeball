use bevy::prelude::*;
use rand::random;

#[derive(Component)]
pub struct UFO {
    pub fall_speed: f32,
    pub size: Vec3, 
    pub score: u32, // zero score mean enemy
    pub kind: UfoType,
}

pub const ENEMY_SPEED: f32 = 150.0;
pub const EXTRA_SPEED_PER_LV: f32 = 30.;
pub const MIN_MODIFIER: f32 = 0.5;
pub const MAX_MODIFIER: f32 = 1.5;

pub enum UfoType {
    ENEMY,
    BONUS,
}

trait RandomFreeFall {
    fn get_speed_modifier(max: f32, min: f32) -> f32 {
        random::<f32>() * (max - min) + min
    }
}

impl RandomFreeFall for UFO {}

impl UFO {
    pub fn new(level: f32, width: f32, height: f32, score: u32, kind: UfoType) -> Self {
        let base_speed = ENEMY_SPEED + level * EXTRA_SPEED_PER_LV;
        let modifier = Self::get_speed_modifier(MAX_MODIFIER, MIN_MODIFIER);
        UFO {
            score, kind,
            fall_speed: base_speed * modifier,
            size: Vec3::new(width, height, 0.),
        }
    }
}

pub trait CollisionObject {
    fn get_dimension(&self) -> (f32, f32);
    
    fn min_distance(&self, target: & impl CollisionObject) -> (f32, f32) {
        let (my_width, my_height) = self.get_dimension();
        let (target_width, target_height) = target.get_dimension();

        ((my_width + target_width) / 2., (my_height + target_height) / 2.) 
    }

    fn collide(
        &self, 
        curr_translation: Vec3, 
        target: & impl CollisionObject, 
        target_translation: Vec3
    ) -> bool {
        let (min_x, min_y) = self.min_distance(target);

        (curr_translation.x - target_translation.x).abs() < min_x
        && (curr_translation.y - target_translation.y).abs() < min_y
    }
}

impl CollisionObject for UFO {
    fn get_dimension(&self) -> (f32, f32) {
        (self.size.x, self.size.y)
    }
}
