pub mod components;
pub mod resources;
pub mod systems;

use bevy::{prelude::{Plugin, App}, app::{Startup, Update}};

use self::{systems::{spawn_enemy_overtime, init_enemy_texture, tick_enemy_spawn_timer, ufo_fall, ufo_cleanup, tick_bonus_spawn_timer, init_bonus_texture, spawn_bonus_overtime}, resources::{EnemySpawnTimer, BonusObjectSpawnTimer}};

pub struct EnemyPlug;

impl Plugin for EnemyPlug {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .init_resource::<BonusObjectSpawnTimer>()
            .add_systems(Startup, (init_enemy_texture, init_bonus_texture))
            .add_systems(Update, (tick_enemy_spawn_timer, tick_bonus_spawn_timer))
            .add_systems(Update, (spawn_enemy_overtime, spawn_bonus_overtime))
            .add_systems(Update, ufo_fall)
            .add_systems(Update, ufo_cleanup);
    }
}