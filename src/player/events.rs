use bevy::prelude::*;

#[derive(Event)]
pub struct LoseLifeEvent {}

#[derive(Event)]
pub struct EarnPointEvent {
    pub scores: u32,
}