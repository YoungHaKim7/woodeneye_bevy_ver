use bevy::prelude::*;

use crate::components::Edges;

pub fn init_edges(scale: i32) -> Edges {
    let r = scale as f32;

    let map = [
        0, 1, 1, 3, 3, 2, 2, 0, // bottom
        7, 6, 6, 4, 4, 5, 5, 7, // top
        6, 2, 3, 7, 0, 4, 5, 1, // verticals
    ];

    let mut edges: Vec<(Vec3, Vec3)> = Vec::with_capacity((12 + scale as usize * 2) as usize);

    for i in 0..12 {
        let mut a = Vec3::ZERO;
        let mut b = Vec3::ZERO;
        for j in 0..3 {
            a[j] = if (map[i * 2] & (1 << j)) != 0 { r } else { -r };
            b[j] = if (map[i * 2 + 1] & (1 << j)) != 0 { r } else { -r };
        }
        edges.push((a, b));
    }

    for i in 0..scale as usize {
        let d = (i as f32) * 2.0;
        for j in 0..2 {
            // wall 1
            let x = if j != 0 { r } else { -r };
            let y = -r;
            let z = d - r;
            let a = Vec3::new(x, y, z);
            let b = Vec3::new(if j == 0 { r } else { -r }, y, z);
            edges.push((a, b));

            // wall 2
            let x2 = d - r;
            let z2 = if j != 0 { r } else { -r };
            let a2 = Vec3::new(x2, y, z2);
            let b2 = Vec3::new(x2, y, if j == 0 { r } else { -r });
            edges.push((a2, b2));
        }
    }

    Edges(edges)
}
