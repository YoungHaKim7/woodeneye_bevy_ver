use bevy::prelude::*;

use crate::components::{Player, Velocity};
use crate::constants::MAX_PLAYER_COUNT;

pub fn setup_players(mut commands: Commands) {
    // Optional light (not required for gizmos, keep for completeness)
    commands.spawn((
        DirectionalLight::default(),
        Transform::default(),
        GlobalTransform::default(),
    ));

    for i in 0..MAX_PLAYER_COUNT {
        let pos = Vec3::new(
            8.0 * if i & 1 != 0 { -1.0 } else { 1.0 },
            0.0,
            8.0 * if i & 1 != 0 { -1.0 } else { 1.0 } * if i & 2 != 0 { -1.0 } else { 1.0 },
        );

        let mut r = 0xffu8;
        let mut g = 0xffu8;
        let mut b = 0xffu8;
        if (1 << (i / 2)) & 2 != 0 {
            r = 0
        };
        if (1 << (i / 2)) & 1 != 0 {
            g = 0
        };
        if (1 << (i / 2)) & 4 != 0 {
            b = 0
        };
        if i & 1 == 0 {
            r = !r;
            g = !g;
            b = !b;
        }
        let color = Color::srgb_u8(r, g, b);

        commands.spawn((
            Player {
                id: i,
                yaw: 0.5 * std::f32::consts::PI
                    + if i & 1 != 0 { std::f32::consts::PI } else { 0.0 }
                    + if i & 2 != 0 { 0.5 * std::f32::consts::PI } else { 0.0 },
                pitch: -0.25 * std::f32::consts::PI,
                radius: 0.5,
                height: 1.5,
                color,
            },
            Velocity(Vec3::ZERO),
            Transform::from_translation(pos),
            GlobalTransform::default(),
        ));
    }
}
