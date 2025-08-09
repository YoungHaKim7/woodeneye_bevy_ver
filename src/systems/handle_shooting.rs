use bevy::prelude::*;
use rand::Rng;

use crate::components::{AppMode, GameMode, Player};
use crate::constants::MAP_BOX_SCALE;

pub fn handle_shooting(
    mode: Res<GameMode>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut q_players: Query<(&mut Transform, &mut Player)>,
) {
    if !matches!(mode.0, AppMode::Playing) {
        return;
    }
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let (origin, yaw, pitch, shooter_id) = {
        let (shooter_tf, shooter) = q_players.iter().find(|(_, p)| p.id == 0).unwrap();
        (shooter_tf.translation, shooter.yaw, shooter.pitch, shooter.id)
    };

    let (sin_yaw, cos_yaw) = yaw.sin_cos();
    let (sin_pitch, cos_pitch) = pitch.sin_cos();
    let dir = Vec3::new(-sin_yaw * cos_pitch, sin_pitch, -cos_yaw * cos_pitch).normalize();

    for (mut tf, mut target) in &mut q_players {
        if target.id == shooter_id {
            continue;
        }
        let offset = tf.translation - origin;
        let mut hit_count = 0;
        for j in 0..2 {
            let dy = offset.y + if j == 0 { 0.0 } else { target.radius - target.height };
            let d = Vec3::new(offset.x, dy, offset.z);
            let vd = dir.dot(d);
            if vd < 0.0 {
                continue;
            }
            let dd = d.length_squared();
            let rr = target.radius * target.radius;
            let vv = 1.0;
            if vd * vd >= vv * (dd - rr) {
                hit_count += 1;
            }
        }
        if hit_count > 0 {
            let scale = MAP_BOX_SCALE as f32;
            let mut rng = rand::thread_rng();
            tf.translation.x = scale * (rng.r#gen::<f32>() - 0.5);
            tf.translation.y = scale * (rng.r#gen::<f32>() - 0.5);
            tf.translation.z = scale * (rng.r#gen::<f32>() - 0.5);
        }
    }
}
