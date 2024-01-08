use bevy::prelude::*;

#[derive(Resource, Clone, Copy)]
pub struct GameBonudary {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub x_mid: f32,
    pub y_mid: f32,
}

#[derive(Resource)]
pub struct GameMetadata {
    pub level: f32,
    pub lifes: u8,
    pub scores: u32,
}

impl Default for GameMetadata {
    fn default() -> Self {
        GameMetadata {
            level: 1.,
            lifes: 3,
            scores: 0,
        }
    }
}