use bevy::{
    prelude::*, 
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Paused,
}

// Components for the game
pub struct Paddle {
    pub speed: f32,
}

pub struct Ball {
    pub velocity: Vec3,
}

pub struct Scoreboard {
    pub score: isize,
}

pub enum Collider {
    Solid,
    Bottom,
    Paddle,
}