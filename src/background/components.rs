use bevy::prelude::Component;

#[derive(Component)]
pub struct Grass {}

#[derive(Component)]
pub struct LifeIcon {
    pub count: u8,
}

#[derive(Component)]
pub struct CurrentScoreRoot;

#[derive(Component)]
pub struct CurrentScore;