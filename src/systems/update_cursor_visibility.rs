use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};

use crate::components::{AppMode, GameMode};

pub fn update_cursor_visibility(mode: Res<GameMode>, mut q: Query<&mut Window>) {
    if let Ok(mut window) = q.single_mut() {
        match mode.0 {
            AppMode::Playing => {
                window.cursor_options = CursorOptions {
                    visible: false,
                    grab_mode: CursorGrabMode::Locked,
                    ..default()
                };
            }
            _ => {
                window.cursor_options = CursorOptions {
                    visible: true,
                    grab_mode: CursorGrabMode::None,
                    ..default()
                };
            }
        }
    }
}
