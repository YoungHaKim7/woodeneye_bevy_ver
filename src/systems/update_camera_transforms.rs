use bevy::prelude::*;

use crate::components::{Player, PlayerCamera};

pub fn update_camera_transforms(
    q_players: Query<(&Transform, &Player)>,
    mut q_cams: Query<(&mut Transform, &PlayerCamera), (With<Camera>, Without<Player>)>,
) {
    for (mut cam_tf, cam) in &mut q_cams {
        if let Some((player_tf, player)) = q_players.iter().find(|(_, p)| p.id == cam.player_id) {
            let cam_pos = player_tf.translation + Vec3::Y * 0.5;
            let (sin_yaw, cos_yaw) = player.yaw.sin_cos();
            let (sin_pitch, cos_pitch) = player.pitch.sin_cos();
            let dir = Vec3::new(-sin_yaw * cos_pitch, sin_pitch, -cos_yaw * cos_pitch);
            *cam_tf = Transform::from_translation(cam_pos).looking_at(cam_pos + dir, Vec3::Y);
        }
    }
}
