use bevy::prelude::*;

use crate::components::{AppMode, GameMode};

pub fn esc_menu_toggle(kb: Res<ButtonInput<KeyCode>>, mut mode: ResMut<GameMode>) {
    if kb.just_pressed(KeyCode::Escape) {
        mode.0 = match mode.0 {
            AppMode::Playing => AppMode::Menu,
            _ => AppMode::Playing,
        };
    }
}
