use bevy::{
    core_pipeline::prelude::Camera3d,
    prelude::*,
    render::camera::{PerspectiveProjection, Projection},
};

use crate::components::PlayerCamera;
use crate::components::PlayerCount;
use crate::constants::MAX_PLAYER_COUNT;

pub fn setup_cameras(mut commands: Commands, player_count: Res<PlayerCount>) {
    // Spawn one camera per potential player; activate based on PlayerCount
    for i in 0..MAX_PLAYER_COUNT {
        commands.spawn((
            Camera {
                is_active: i < player_count.0,
                ..default()
            },
            Camera3d::default(),
            Projection::Perspective(PerspectiveProjection::default()),
            Transform::from_translation(Vec3::new(0.0, 2.0, 5.0)).looking_at(Vec3::ZERO, Vec3::Y),
            GlobalTransform::default(),
            PlayerCamera { player_id: i },
        ));
    }
}
