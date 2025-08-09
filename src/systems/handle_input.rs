use bevy::prelude::*;

use crate::components::{AppMode, GameMode, MouseDelta, Player, PlayerCount, SettingsRes};

pub fn handle_input(
    mode: Res<GameMode>,
    kb: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Player>,
    mut player_count: ResMut<PlayerCount>,
    mut mouse_delta: ResMut<MouseDelta>,
    settings: Res<SettingsRes>,
) {
    if !matches!(mode.0, AppMode::Playing) {
        return;
    }
    // Expand players up to MAX by pressing keys 1..4 (optional join)
    if kb.just_pressed(KeyCode::Digit1) {
        player_count.0 = player_count.0.max(1);
    }
    if kb.just_pressed(KeyCode::Digit2) {
        player_count.0 = player_count.0.max(2);
    }
    if kb.just_pressed(KeyCode::Digit3) {
        player_count.0 = player_count.0.max(3);
    }
    if kb.just_pressed(KeyCode::Digit4) {
        player_count.0 = player_count.0.max(4);
    }

    // Mouse-look and WASD for player 0 (simplified mapping)
    let mut p0 = query.iter_mut().find(|p| p.id == 0).unwrap();

    let sens = settings.sensitivity;

    p0.yaw -= mouse_delta.dx * sens;
    p0.pitch -= mouse_delta.dy * sens;

    let clamp = 1.6f32;
    p0.pitch = p0.pitch.clamp(-clamp, clamp);

    mouse_delta.dx = 0.0;
    mouse_delta.dy = 0.0;

    // Additional players keys are handled in physics
    for mut _p in &mut query {
        // handled elsewhere
    }
}
