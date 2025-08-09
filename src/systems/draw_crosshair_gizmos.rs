use bevy::prelude::*;

use crate::components::{AppMode, GameMode, SettingsRes};

pub fn draw_crosshair_gizmos(
    mode: Res<GameMode>,
    settings: Res<SettingsRes>,
    mut gizmos: Gizmos,
    cams: Query<(&Transform, &Camera)>,
) {
    if !matches!(mode.0, AppMode::Playing) {
        return;
    }
    for (tf, cam) in &cams {
        if !cam.is_active {
            continue;
        }
        let origin = tf.translation + tf.forward() * 2.0;
        let right = tf.right() * settings.crosshair_half;
        let up = tf.up() * settings.crosshair_half;
        gizmos.line(origin - right, origin + right, Color::WHITE);
        gizmos.line(origin - up, origin + up, Color::WHITE);
    }
}
