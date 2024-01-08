use crate::{
    player::{
        systems::{
            CHARACTER_WIDTH, CHARACTER_HEIGHT
        },
        events::{LoseLifeEvent, EarnPointEvent}
    }, 
    systems::GameOver
};

use super::{
    components::{
        Grass, LifeIcon, CurrentScore, CurrentScoreRoot
    }, 
    resources::{
        GameBonudary, GameMetadata
    }
};

use bevy::{prelude::*, window::PrimaryWindow};

const OVERLAY_IDX: f32 = 5.;

const GRASS_WIDTH_ORIGINAL: f32 = 51.0;
const GRASS_HEIGHT_ORIGINAL: f32 = 50.0;
const GRASS_SCALE: f32 = CHARACTER_WIDTH / GRASS_WIDTH_ORIGINAL;
const GRASS_WIDTH_SCALED: f32 = GRASS_WIDTH_ORIGINAL * GRASS_SCALE;
const GRASS_HEIGHT_SCALED: f32 = GRASS_HEIGHT_ORIGINAL * GRASS_SCALE;

// one extra row and col is given. but they are not accessible as it reflects the character size
pub const WINDOW_WIDTH: f32 = CHARACTER_WIDTH * 7.;
pub const WINDOW_HEIGHT: f32 = CHARACTER_WIDTH * 9.; 

fn get_game_boundary(window: &Window) -> GameBonudary {
    let x_mid = window.width() / 2.; 
    let y_mid = window.height() / 2.; 
    let x_min = x_mid - WINDOW_WIDTH / 2.;
    let x_max = x_mid + WINDOW_WIDTH / 2.;
    let y_min = y_mid - WINDOW_HEIGHT / 2.;
    let y_max = y_mid + WINDOW_HEIGHT / 2.;

    GameBonudary {
        x_max, x_min, y_max, y_min, x_mid, y_mid
    }
}

pub fn spawn_grass(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    
    // find mid pt of the window
    let game_boundary = get_game_boundary(window);
    commands.insert_resource(game_boundary);

    // x, y of the cursor in the x,y grid, will be used in the loop below
    // initially at the bottom left corner
    let (mut x_curr, mut y_curr) = (game_boundary.x_min, game_boundary.y_min);

    // find the x,y coordinate of the background block
    while x_curr <= game_boundary.x_max {
        // from left to right
        while y_curr <= game_boundary.y_max {
            // from bottom to top
            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(x_curr, y_curr, -1.),
                        scale: Vec3::new(GRASS_SCALE, GRASS_SCALE, GRASS_SCALE),
                        ..default()
                    },
                    texture: asset_server.load("sprites/background/slimeBlock.png"),
                    ..default()
                },
                Grass {},
            ));
            y_curr += GRASS_HEIGHT_SCALED;
        }
        y_curr = game_boundary.y_min;
        x_curr += GRASS_WIDTH_SCALED;
    }
}

const LIFE_ICON_HEIGHT: f32 = 50.;
const LIFE_ICON_SCALE: f32 = LIFE_ICON_HEIGHT / CHARACTER_HEIGHT;
const LIFE_ICON_WIDTH: f32 = CHARACTER_WIDTH * LIFE_ICON_SCALE;

pub fn spawn_lifes(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    game_metadata: Res<GameMetadata>,
) {
    let window = window_query.get_single().unwrap();
    let game_boundary = get_game_boundary(window);

    for i in 0..game_metadata.lifes {
        // spawn life icon
        let texture: Handle<Image> = asset_server.load("sprites/characters/alienYellow_walk1.png");
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        game_boundary.x_min + LIFE_ICON_WIDTH * 1.2 * (i as f32),
                        game_boundary.y_max - LIFE_ICON_HEIGHT / 2.,
                        OVERLAY_IDX,
                    ),
                    scale: Vec3::new(LIFE_ICON_SCALE, LIFE_ICON_SCALE, LIFE_ICON_SCALE),
                    ..default()
                },
                texture,
                ..default()
            }, 
            LifeIcon {
                count: i,
            }
        ));
    }
}

pub fn show_scores(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    game_metadata: Res<GameMetadata>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    let game_boundary = get_game_boundary(window);

    let font_size = 40.0;

    let root = commands.spawn((
        CurrentScoreRoot,
        NodeBundle {
            // give it a dark background for readability
            background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
            // make it "always on top" by setting the Z index to maximum
            // we want it to be displayed over all other UI
            z_index: ZIndex::Global(i32::MAX),
            style: Style {
                position_type: PositionType::Absolute,
                // position it at the top-right corner
                // 1% away from the top window edge
                right: Val::Auto,
                top: Val::Auto,
                align_content: AlignContent::End,
                // set bottom/left to Auto, so it can be
                // automatically sized depending on the text
                bottom: Val::Px(game_boundary.y_max - font_size),
                width: Val::Px(WINDOW_WIDTH / 2.),
                left: Val::Px(game_boundary.x_min + WINDOW_WIDTH / 2.),
                // give it some padding for readability
                padding: UiRect::all(Val::Px(4.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    )).id();
    // show score icon on the top right
    let text_fps = commands.spawn((
        CurrentScore,
        TextBundle {
            // use two sections, so it is easy to update just the number
            text: Text::from_sections([
                TextSection {
                    value: game_metadata.scores.to_string(),
                    style: TextStyle {
                        font_size,
                        color: Color::WHITE,
                        font: asset_server.load("BungeeSpice-Regular.ttf"),
                        ..default()
                    }
                },
            ]).with_alignment(TextAlignment::Right),
            ..Default::default()
        },
    )).id();
    commands.entity(root).push_children(&[text_fps]);

}

// subscribe to LoseLifeEvent
pub fn update_user_life(
    mut commands: Commands,
    mut lose_life_events: EventReader<LoseLifeEvent>,
    mut game_over_event: EventWriter<GameOver>,
    life_icon_query: Query<(Entity, &LifeIcon)>,
    mut game_metadata: ResMut<GameMetadata>,
) {
    /* @todo check game over */
    for _ in lose_life_events.read() {
        game_metadata.lifes -= 1;
        
        for (entity, life_icon) in life_icon_query.iter() {   
            if life_icon.count >= game_metadata.lifes {
                commands.entity(entity).despawn()
            }
        }
    }

    if game_metadata.lifes <= 0 {
        game_over_event.send(GameOver {
            score: 10,
        })   
    }
}

pub fn update_points(
    mut earn_point_events: EventReader<EarnPointEvent>,
    mut game_metadata: ResMut<GameMetadata>,
    mut current_score_query: Query<&mut Text, With<CurrentScore>>
) {
    /* @todo check game over */
    for event in earn_point_events.read() {
        game_metadata.scores += event.scores;
        // update text display
        for mut text in &mut current_score_query {
            text.sections[0].value = game_metadata.scores.to_string();
        };
    }
}
