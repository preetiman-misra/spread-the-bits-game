use bevy::{prelude::*, window::PrimaryWindow};

use crate::{CELL_SIZE, OFFSET_X, OFFSET_Y, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_board)
            .add_systems(Update, (set_active_cell, highlight_active_cell));
    }
}

/// A `Cell` represents a single tile on the `Board`
/// in the game world.
#[derive(Debug, Component)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
    pub is_active: bool,
}

/// Draw the game board.
fn setup_board(mut commands: Commands) {
    // Spawn Camera2dBundle
    commands.spawn(Camera2dBundle::default());

    for row in 0..10 {
        for col in 0..10 {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(CELL_SIZE - 1., CELL_SIZE - 1.)),
                        color: Color::WHITE,
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        // Draw the cells from top left
                        // instead of the window center.
                        col as f32 * CELL_SIZE + OFFSET_X,
                        -(row as f32) * CELL_SIZE + OFFSET_Y,
                        0.,
                    )),
                    ..default()
                },
                Cell {
                    row: row as usize,
                    col: col as usize,
                    is_active: false,
                },
            ));
        }
    }
}

/// Set `cell.is_active` to `true` when
/// the mouse cursor hovers within a `Cell`'s area.
fn set_active_cell(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut queries: Query<(&Transform, &mut Cell)>,
) {
    if let Some(position) = q_windows.single().cursor_position() {
        // Mouse position is a `Vec2` with origin at
        // the top left of the window. Origin shifting
        // has to be done in order to match coordinate
        // systems with cell transform.
        let (mx, my) = (
            position.x - (WINDOW_WIDTH / 2.),
            -position.y + (WINDOW_HEIGHT / 2.),
        );

        for (transform, mut cell) in queries.iter_mut() {
            let (a, b) = (transform.translation.x, transform.translation.y);

            // Change cell state to active
            // if the mouse cursor is within
            // the cell's area.
            cell.is_active = (mx > (a - CELL_SIZE / 2.) && mx < (a + CELL_SIZE / 2.))
                && (my > (b - CELL_SIZE / 2.) && my < (b + CELL_SIZE / 2.));
        }
    }
}

/// Set the `Cell` color to `Color::RED`
/// when `cell.is_active = true`.
fn highlight_active_cell(mut queries: Query<(&mut Sprite, &Cell)>) {
    for (mut sprite, cell) in queries.iter_mut() {
        sprite.color = if cell.is_active {
            Color::RED
        } else {
            Color::WHITE
        };
    }
}
