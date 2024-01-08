use crate::background::{
    resources::{
        GameBonudary, GameMetadata
    }, 
    systems::WINDOW_WIDTH
};

use super::{
    components::{UFO, UfoType}, 
    resources::{
        EnemyTexture, EnemySpawnTimer, BonusObjectSpawnTimer, BonusObjectTexture
    }
};

use bevy::prelude::*;
use rand::random;

pub const UFO_WIDTH: f32 = 51.0;
pub const UFO_HEIGHT: f32 = 73.0;

pub fn init_enemy_texture(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let handles: Vec<Handle<Image>> = [
        asset_server.load("sprites/enemy/ghost_normal.png"),
        asset_server.load("sprites/enemy/ghost.png"),
    ].to_vec();
    
    commands.insert_resource(EnemyTexture { handles });
}

pub fn init_bonus_texture(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let handles: Vec<Handle<Image>> = [
        asset_server.load("sprites/enemy/slime.png"),
        asset_server.load("sprites/enemy/slimeBlue.png"),
        asset_server.load("sprites/enemy/slimeGreen.png"),
    ].to_vec();
    
    commands.insert_resource(BonusObjectTexture { handles });
}

pub fn tick_enemy_spawn_timer(mut spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    spawn_timer.timer.tick(time.delta());
}

pub fn tick_bonus_spawn_timer(mut spawn_timer: ResMut<BonusObjectSpawnTimer>, time: Res<Time>) {
    spawn_timer.timer.tick(time.delta());
}

fn spawn_ufo(
    mut commands: Commands,
    texture: Handle<Image>,
    game_boundary: &GameBonudary,
    game_metadata: &GameMetadata,
    score: u32,
    kind: UfoType,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                random::<f32>() * WINDOW_WIDTH + game_boundary.x_min,
                game_boundary.y_max,
                0.,
            ),
            texture,
            ..default()
        },
        UFO::new(game_metadata.level, UFO_WIDTH, UFO_HEIGHT, score, kind),
    ));
}

fn pick_texture(handles: &Vec<Handle<Image>>) -> Handle<Image> {
    let texture_idx: usize = (handles.len() - 1) * (random::<f32>() as usize);
    handles[texture_idx].clone()
}

pub fn spawn_enemy_overtime(
    commands: Commands,
    texture: Res<EnemyTexture>,
    spawn_timer: Res<EnemySpawnTimer>,
    game_boundary: Res<GameBonudary>,
    game_metadata: Res<GameMetadata>,
) {
    if spawn_timer.timer.finished() {
        // spawn new
        let selected_texture = pick_texture(&texture.handles);
        spawn_ufo(commands, selected_texture, game_boundary.as_ref(), game_metadata.as_ref(), 0, UfoType::ENEMY);
    }
}

pub fn spawn_bonus_overtime(
    commands: Commands,
    texture: Res<BonusObjectTexture>,
    spawn_timer: Res<BonusObjectSpawnTimer>,
    game_boundary: Res<GameBonudary>,
    game_metadata: Res<GameMetadata>,
) {
    if spawn_timer.timer.finished() {
        // spawn new
        let selected_texture = pick_texture(&texture.handles);
        spawn_ufo(commands, selected_texture, game_boundary.as_ref(), game_metadata.as_ref(), 10, UfoType::BONUS);
    }
}

pub fn ufo_fall(
    mut ufo_query: Query<(&mut Transform, & UFO)>,
    time: Res<Time>,
) {
    for (mut transform, enemy ) in ufo_query.iter_mut() {
        let direction = Vec3::new(0., -1., 0.);
        transform.translation += direction * enemy.fall_speed * time.delta_seconds();
    }
}

pub fn ufo_cleanup(
    mut command: Commands,
    ufo_query: Query<(Entity, &Transform), With<UFO>>,
    game_boundary: Res<GameBonudary>,
) {
    for (entity, transform) in ufo_query.iter() {
        if transform.translation.y < game_boundary.y_min {
            command.entity(entity).despawn();
        }
    }
}