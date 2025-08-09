use bevy::prelude::*;

use crate::components::{AppMode, GameMode, MouseDelta};

pub fn accumulate_mouse_motion(
    mode: Res<GameMode>,
    mut ev_motion: EventReader<bevy::input::mouse::MouseMotion>,
    mut delta: ResMut<MouseDelta>,
) {
    if !matches!(mode.0, AppMode::Playing) {
        return;
    }
    let mut dx = 0.0f32;
    let mut dy = 0.0f32;
    for e in ev_motion.read() {
        dx += e.delta.x;
        dy += e.delta.y;
    }
    delta.dx = dx;
    delta.dy = dy;
}
