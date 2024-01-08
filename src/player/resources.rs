use bevy::prelude::*;

#[derive(Resource)]
pub struct CharacterTexture {
    pub head: Handle<Image>,
    pub tail: Handle<Image>
}