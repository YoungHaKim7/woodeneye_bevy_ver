use bevy::prelude::*;

use crate::components::{AppMode, GameMode, Player, Velocity};
use crate::constants::{DRAG_RATE, GRAVITY, JUMP_VELOCITY, MAP_BOX_SCALE, MOVE_MULT};

pub fn update_physics(
    mode: Res<GameMode>,
    time: Res<Time>,
    kb: Res<ButtonInput<KeyCode>>,
    mut q: Query<(&mut Transform, &mut Velocity, &mut Player)>,
) {
    if !matches!(mode.0, AppMode::Playing) {
        return;
    }
    let dt = time.delta_secs().max(1e-6);
    let drag = (-dt * DRAG_RATE).exp();
    let diff = 1.0 - drag;

    for (mut transform, mut vel, mut player) in &mut q {
        let (up, left, down, right, jump) = match player.id {
            0 => (KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD, KeyCode::Space),
            1 => (
                KeyCode::ArrowUp,
                KeyCode::ArrowLeft,
                KeyCode::ArrowDown,
                KeyCode::ArrowRight,
                KeyCode::Numpad0,
            ),
            2 => (
                KeyCode::KeyI,
                KeyCode::KeyJ,
                KeyCode::KeyK,
                KeyCode::KeyL,
                KeyCode::ShiftRight,
            ),
            _ => (
                KeyCode::Numpad8,
                KeyCode::Numpad4,
                KeyCode::Numpad5,
                KeyCode::Numpad6,
                KeyCode::NumpadEnter,
            ),
        };

        let dir_x = (kb.pressed(right) as i8 - kb.pressed(left) as i8) as f32;
        let dir_z = (kb.pressed(up) as i8 - kb.pressed(down) as i8) as f32;
        let norm = (dir_x * dir_x + dir_z * dir_z).sqrt();
        let (sin_yaw, cos_yaw) = player.yaw.sin_cos();
        let acc_x = MOVE_MULT
            * if norm == 0.0 {
                0.0
            } else {
                (cos_yaw * dir_x + sin_yaw * dir_z) / norm
            };
        let acc_z = MOVE_MULT
            * if norm == 0.0 {
                0.0
            } else {
                (-sin_yaw * dir_x + cos_yaw * dir_z) / norm
            };

        vel.x -= vel.x * diff;
        vel.z -= vel.z * diff;
        vel.y -= GRAVITY * dt;

        vel.x += diff * acc_x / DRAG_RATE;
        vel.z += diff * acc_z / DRAG_RATE;

        transform.translation.x += (dt - diff / DRAG_RATE) * acc_x / DRAG_RATE + diff * vel.x / DRAG_RATE;
        transform.translation.y += -0.5 * GRAVITY * dt * dt + vel.y * dt;
        transform.translation.z += (dt - diff / DRAG_RATE) * acc_z / DRAG_RATE + diff * vel.z / DRAG_RATE;

        let scale = MAP_BOX_SCALE as f32;
        let bound = scale - player.radius;
        let mut pos = transform.translation;
        let mut hit_x = false;
        let mut hit_y = false;
        let mut hit_z = false;
        if pos.x < -bound {
            pos.x = -bound;
            hit_x = true;
        }
        if pos.x > bound {
            pos.x = bound;
            hit_x = true;
        }
        if pos.y < player.height - scale {
            pos.y = player.height - scale;
            hit_y = true;
        }
        if pos.y > bound {
            pos.y = bound;
            hit_y = true;
        }
        if pos.z < -bound {
            pos.z = -bound;
            hit_z = true;
        }
        if pos.z > bound {
            pos.z = bound;
            hit_z = true;
        }
        if hit_x {
            vel.x = 0.0;
        }
        if hit_z {
            vel.z = 0.0;
        }
        if hit_y {
            vel.y = if kb.pressed(jump) { JUMP_VELOCITY } else { 0.0 };
        }
        transform.translation = pos;
    }
}
