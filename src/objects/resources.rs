use bevy::prelude::*;

const ENEMY_DEFAULT_SPAWN_DURATION: f32 = 1.0;
const BONUS_DEFAULT_SPAWN_DURATION: f32 = ENEMY_DEFAULT_SPAWN_DURATION * 2.;

#[derive(Resource)]
pub struct EnemyTexture {
    pub handles: Vec<Handle<Image>>
}

#[derive(Resource)]
pub struct BonusObjectTexture {
    pub handles: Vec<Handle<Image>>
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_DEFAULT_SPAWN_DURATION, TimerMode::Repeating)
        }
    }
}

#[derive(Resource)]
pub struct BonusObjectSpawnTimer {
    pub timer: Timer
}

impl Default for BonusObjectSpawnTimer {
    fn default() -> Self {
        BonusObjectSpawnTimer {
            timer: Timer::from_seconds(BONUS_DEFAULT_SPAWN_DURATION, TimerMode::Repeating)
        }
    }
}
