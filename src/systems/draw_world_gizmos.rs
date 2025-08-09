use bevy::prelude::*;

use crate::components::{AppMode, Edges, GameMode, Player};

pub fn draw_world_gizmos(
    mode: Res<GameMode>,
    edges: Res<Edges>,
    q_players: Query<(&Transform, &Player)>,
    mut gizmos: Gizmos,
) {
    if !matches!(mode.0, AppMode::Playing) {
        return;
    }
    let edge_color = Color::srgb(0.25, 0.25, 0.25);
    for (a, b) in &edges.0 {
        gizmos.line(*a, *b, edge_color);
    }

    for (tf, p) in &q_players {
        let feet = tf.translation + Vec3::Y * (p.radius - p.height);
        let head = tf.translation;
        gizmos.line(feet, head, p.color);
        gizmos.sphere(feet, p.radius, p.color);
        gizmos.sphere(head, p.radius, p.color);
    }
}
