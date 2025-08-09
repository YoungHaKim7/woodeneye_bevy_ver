use bevy::prelude::*;

use crate::components::PlayerCount;

pub fn update_camera_viewports(
    windows: Query<&Window>,
    mut q_cams: Query<(&mut Camera, &crate::components::PlayerCamera)>,
    player_count: Res<PlayerCount>,
) {
    let window = windows.single().expect("primary window");
    let (w, h) = (
        window.resolution.physical_width(),
        window.resolution.physical_height(),
    );
    let (part_hor, part_ver): (usize, usize) = if player_count.0 > 2 {
        (2, 2)
    } else if player_count.0 > 1 {
        (2, 1)
    } else {
        (1, 1)
    };
    let size_w: u32 = w / part_hor as u32;
    let size_h: u32 = h / part_ver as u32;

    for (mut cam, cam_tag) in &mut q_cams {
        let i = cam_tag.player_id;
        let active = i < player_count.0;
        cam.is_active = active;
        if !active {
            cam.viewport = None;
            continue;
        }
        let mod_x = (i % part_hor) as u32;
        let mod_y = (i / part_hor) as u32;
        cam.viewport = Some(bevy::render::camera::Viewport {
            physical_position: UVec2::new(mod_x * size_w, mod_y * size_h),
            physical_size: UVec2::new(size_w, size_h),
            depth: 0.0..1.0,
        });
    }
}
