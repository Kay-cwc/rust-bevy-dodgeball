pub mod components;
pub mod resources;
pub mod systems;
pub mod events;

use bevy::{prelude::{Plugin, App}, app::{Startup, Update}};

use self::{systems::{spawn_character, character_movement, init_character_texture, obj_collision}, events::{LoseLifeEvent, EarnPointEvent}};

pub struct CharacterPlug;

impl Plugin for CharacterPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_character)
            .add_systems(Startup, init_character_texture)
            .add_event::<LoseLifeEvent>()
            .add_event::<EarnPointEvent>()
            .add_systems(Update, character_movement)
            .add_systems(Update, obj_collision);
    }
}