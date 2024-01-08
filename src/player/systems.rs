use crate::{
    background::resources::GameBonudary, 
    objects::components::{
        UFO, CollisionObject, UfoType
    }
};
use super::{components::Character, resources::CharacterTexture, events::{LoseLifeEvent, EarnPointEvent}};

use bevy::{prelude::*, window::PrimaryWindow};

pub const CHARACTER_HEIGHT: f32 = 84.;
pub const CHARACTER_WIDTH: f32 = 70.;
const CHARACTER_MOVEMENT_SPEED: f32 = 300.0;
const CHARACTER_TEXTURE_FLIP_FREQ: f32 = 0.25;
const INVULNERABLE_DURATION: f32 = 3.;

pub fn init_character_texture(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let head: Handle<Image> = asset_server.load("sprites/characters/alienYellow_walk2.png");
    let tail: Handle<Image> = asset_server.load("sprites/characters/alienYellow_walk1.png");

    commands.insert_resource(CharacterTexture { head, tail });
}

pub fn spawn_character(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    media_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // random spawn character on the lower 1/3 of the window
    let x = window.width() / 2.;
    let y = CHARACTER_HEIGHT * 1.5;

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.),
            texture: media_server.load("sprites/characters/alienYellow_walk2.png"),
            ..default()
        },
        Character { ..default() }
    ));
}

// given an new xyz and the window frame, return an possible xyz
fn confine_movement(translation: &Vec3, boundary: &GameBonudary) -> Vec3 {
    let mut output = translation.clone();
    if translation.x > boundary.x_max {
        output.x = boundary.x_max;
    } else if translation.x < boundary.x_min {
        output.x = boundary.x_min;
    } else if translation.y > boundary.y_max {
        output.y = boundary.y_max;
    } else if translation.y < boundary.y_min {
        output.y = boundary.y_min
    }

    output
}

// handle the movement of character in response to keyboard action
pub fn character_movement(
    mut character_query: Query<(&mut Transform, &mut Character, &mut Handle<Image>)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    character_texture: Res<CharacterTexture>,
    game_boundary: Res<GameBonudary>,
) {
    if let Ok((mut transform, mut character, mut texture)) = character_query.get_single_mut() {
        let mut movement_direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Up) {
            movement_direction += Vec3::new(0., 1., 0.);
        }

        if keyboard_input.pressed(KeyCode::Down) {
            movement_direction += Vec3::new(0., -1., 0.);
        }

        if keyboard_input.pressed(KeyCode::Left) {
            movement_direction += Vec3::new(-1., 0., 0.);
        }

        if keyboard_input.pressed(KeyCode::Right) {
            movement_direction += Vec3::new(1., 1., 0.);
        }

        if movement_direction.length() > 0. {
            movement_direction = movement_direction.normalize();
            let elapsed = time.elapsed_seconds();
            if elapsed - character.last_flip > CHARACTER_TEXTURE_FLIP_FREQ {
                if character.is_head {
                    *texture = character_texture.tail.clone();
                } else {
                    *texture = character_texture.head.clone();
                }
                character.is_head = !character.is_head;
                character.last_flip = elapsed;
            }
        }

        let new_translation = transform.translation + movement_direction * CHARACTER_MOVEMENT_SPEED * time.delta_seconds();
        transform.translation = confine_movement(&new_translation, game_boundary.as_ref());
    }
}

// define the behavior when user collide with an ufo
pub fn obj_collision(
    mut commands: Commands,
    mut lose_life_event: EventWriter<LoseLifeEvent>,
    mut earn_point_event: EventWriter<EarnPointEvent>,
    mut character_query: Query<(&Transform, &mut Character)>,
    ufo_query: Query<(Entity, &Transform, &UFO), With<UFO>>,
    time: Res<Time>,
) {
    if let Ok((char_transform, mut character)) = character_query.get_single_mut() {
        for (ufo_entity, ufo_transform, ufo ) in ufo_query.iter() {
            let is_collide = character.collide(
                char_transform.translation, 
                ufo, 
                ufo_transform.translation
            );
            if is_collide {
                match ufo.kind {
                    UfoType::ENEMY => {
                        let now = time.elapsed_seconds();
                        if character.invulnerable_until < now {
                            character.invulnerable_until = time.elapsed_seconds() + INVULNERABLE_DURATION;
                            lose_life_event.send(LoseLifeEvent {});
                        }
                    },
                    UfoType::BONUS => {
                        // update scorse
                        println!("send events");
                        earn_point_event.send(EarnPointEvent {
                            scores: ufo.score,
                        })
                    }
                }

                // despawn the ufo no matter what
                commands.entity(ufo_entity).despawn();
            }
        }
    }
}