pub mod systems;
pub mod components;
pub mod resources;

use self::{
    systems::{
        spawn_grass, spawn_lifes, update_user_life, show_scores, update_points
    }, 
    resources::GameMetadata
};

use bevy::{prelude::{Plugin, App}, app::{Startup, Update}};

pub struct BackgroundPlug;

impl Plugin for BackgroundPlug {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameMetadata>()
            .add_systems(Startup, (spawn_grass, spawn_lifes, show_scores))
            .add_systems(Update, (update_user_life, update_points));
    }
}