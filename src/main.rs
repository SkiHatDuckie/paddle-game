#![windows_subsystem = "windows"]

mod setup;
mod components;
mod movement;

use bevy::{
    prelude::*, 
    render::pass::ClearColor,
};
use components::*;

// Constants
const WIN_WIDTH: f32 = 560.0;
const WIN_HEIGHT: f32 = 560.0;

// Simple paddle game as my introduction to developing AI :]
fn main() {
    App::build()
        .add_plugin(WindowInitPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(setup::SetupPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(pause_system.system())
        .add_system(scoreboard_system.system())
        .run();
}

// Initialize window
struct WindowInitPlugin;

impl Plugin for WindowInitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(WindowDescriptor {
                title: "Simple Paddle Game".to_string(),
                width: WIN_WIDTH,
                height: WIN_HEIGHT,
                vsync: true,
                resizable: false,
                ..Default::default()
            })
            .insert_resource(ClearColor(Color::rgb_u8(10, 25, 30)));
    }
}

fn scoreboard_system(
    scoreboard: Res<Scoreboard>, 
    mut query: Query<&mut Text>
) {
    let mut text = query.single_mut().unwrap();
    text.sections[0].value = format!("Score: {}", scoreboard.score);
}

fn pause_system(
    mut app_state: ResMut<State<AppState>>,
    mut key_input: ResMut<Input<KeyCode>>,
) {
    if key_input.just_pressed(KeyCode::Space) {
        match app_state.current() {
            AppState::Paused => {
                app_state.pop().unwrap();
            }
            AppState::InGame => {
                app_state.push(AppState::Paused).unwrap();
            }
        }
        key_input.reset(KeyCode::Space);
    }
}
