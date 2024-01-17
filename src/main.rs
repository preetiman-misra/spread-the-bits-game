use bevy::{
    prelude::*,
    window::{Window, WindowPlugin},
};

// Global constants
const WINDOW_WIDTH: f32 = 600.;
const WINDOW_HEIGHT: f32 = 600.;
const CELL_SIZE: f32 = WINDOW_WIDTH / 10.;

const OFFSET_X: f32 = (CELL_SIZE / 2.) - (WINDOW_WIDTH / 2.);
const OFFSET_Y: f32 = -(CELL_SIZE / 2.) + (WINDOW_HEIGHT / 2.);

mod board;

use board::BoardPlugin;

fn main() {
    App::new()
        // Set initial window options
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Spread The Bits - 2024".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        // Draw the board
        .add_plugins(BoardPlugin)
        .run();
}
