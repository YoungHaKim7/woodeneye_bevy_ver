use bevy::prelude::*;

use crate::components::{AppMode, GameMode, SettingsRes};

pub fn handle_settings_input(
    mode: Res<GameMode>,
    kb: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<SettingsRes>,
) {
    if !matches!(mode.0, AppMode::Settings) {
        return;
    }
    if kb.just_pressed(KeyCode::BracketLeft) {
        settings.crosshair_half = (settings.crosshair_half - 0.02).max(0.02);
    }
    if kb.just_pressed(KeyCode::BracketRight) {
        settings.crosshair_half = (settings.crosshair_half + 0.02).min(1.0);
    }
    if kb.just_pressed(KeyCode::Minus) {
        settings.sensitivity = (settings.sensitivity - 0.0005).max(0.0001);
    }
    if kb.just_pressed(KeyCode::Equal) {
        settings.sensitivity = (settings.sensitivity + 0.0005).min(0.02);
    }
}
