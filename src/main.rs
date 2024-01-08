mod background;
mod objects;
mod player;
mod systems;

use background::BackgroundPlug;
use objects::EnemyPlug;
use player::CharacterPlug;
use systems::{spawn_camera, exit_game, handle_game_over, GameOver};

use bevy::prelude::*;

fn main() {
    /* @todo handle resize */
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_event::<GameOver>()
        .add_plugins(BackgroundPlug)
        .add_plugins(CharacterPlug)
        .add_plugins(EnemyPlug)
        .add_systems(Update, (exit_game, handle_game_over))
        .run()
}
