use bevy::prelude::*;
use bevy::render::camera::Projection;
use bevy::render::camera::PerspectiveProjection;
use bevy::core_pipeline::prelude::Camera3d;
use rand::Rng;

// --- Constants (ported from SDL version) ---
const MAP_BOX_SCALE: i32 = 16; // half side length in world units
const MAX_PLAYER_COUNT: usize = 4;
const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;
const CROSS_WORLD_HALF: f32 = 0.15; // half length of crosshair arms in world units

// Physics tuning
const DRAG_RATE: f32 = 6.0;
const MOVE_MULT: f32 = 60.0;
const GRAVITY: f32 = 25.0;
const JUMP_VELOCITY: f32 = 8.4375;

// No explicit bitmask needed; we query input each frame.

// --- Components & Resources ---

#[derive(Component)]
struct Player {
    id: usize,
    yaw: f32,   // radians
    pitch: f32, // radians
    radius: f32,
    height: f32,
    color: Color,
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);

#[derive(Component)]
struct PlayerCamera {
    player_id: usize,
}

#[derive(Resource)]
struct PlayerCount(usize);

#[derive(Resource)]
struct Edges(Vec<(Vec3, Vec3)>);

#[derive(Resource, Default)]
struct MouseDelta {
    dx: f32,
    dy: f32,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum CrosshairKind {
    Vertical,
    Horizontal,
}

#[derive(Component)]
struct Crosshair {
    player_id: usize,
    kind: CrosshairKind,
}

// --- Setup ---

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Example splitscreen shooter game (Bevy)".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(PlayerCount(1))
        .insert_resource(MouseDelta::default())
        .insert_resource(init_edges(MAP_BOX_SCALE))
        .add_systems(Startup, (setup_players, setup_cameras))
        .add_systems(
            Update,
            (
                accumulate_mouse_motion,
                handle_input,
                update_physics,
                handle_shooting,
                draw_world_gizmos,
                draw_crosshair_gizmos,
                update_camera_transforms,
                update_camera_viewports,
            ),
        )
        .run();
}

fn setup_players(mut commands: Commands) {
    // Optional light (not required for gizmos, keep for completeness)
    commands.spawn((DirectionalLight::default(), Transform::default(), GlobalTransform::default()));

    for i in 0..MAX_PLAYER_COUNT {
        let pos = Vec3::new(
            8.0 * if i & 1 != 0 { -1.0 } else { 1.0 },
            0.0,
            8.0 * if i & 1 != 0 { -1.0 } else { 1.0 } * if i & 2 != 0 { -1.0 } else { 1.0 },
        );

        let mut r = 0xffu8;
        let mut g = 0xffu8;
        let mut b = 0xffu8;
        if (1 << (i / 2)) & 2 != 0 { r = 0 };
        if (1 << (i / 2)) & 1 != 0 { g = 0 };
        if (1 << (i / 2)) & 4 != 0 { b = 0 };
        if i & 1 == 0 { r = !r; g = !g; b = !b; }
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

fn setup_cameras(mut commands: Commands, player_count: Res<PlayerCount>) {
    // Spawn one camera per potential player; activate based on PlayerCount
    for i in 0..MAX_PLAYER_COUNT {
        commands.spawn((
            Camera { is_active: i < player_count.0, ..default() },
            Camera3d::default(),
            Projection::Perspective(PerspectiveProjection::default()),
            Transform::from_translation(Vec3::new(0.0, 2.0, 5.0)).looking_at(Vec3::ZERO, Vec3::Y),
            GlobalTransform::default(),
            PlayerCamera { player_id: i },
        ));
    }
}

// --- Systems ---

fn accumulate_mouse_motion(mut ev_motion: EventReader<bevy::input::mouse::MouseMotion>, mut delta: ResMut<MouseDelta>) {
    let mut dx = 0.0f32;
    let mut dy = 0.0f32;
    for e in ev_motion.read() {
        dx += e.delta.x;
        dy += e.delta.y;
    }
    delta.dx = dx;
    delta.dy = dy;
}

fn handle_input(
    kb: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Player>,
    mut player_count: ResMut<PlayerCount>,
    mut mouse_delta: ResMut<MouseDelta>,
) {
    // Expand players up to MAX by pressing keys 1..4 (optional join)
    if kb.just_pressed(KeyCode::Digit1) { player_count.0 = player_count.0.max(1); }
    if kb.just_pressed(KeyCode::Digit2) { player_count.0 = player_count.0.max(2); }
    if kb.just_pressed(KeyCode::Digit3) { player_count.0 = player_count.0.max(3); }
    if kb.just_pressed(KeyCode::Digit4) { player_count.0 = player_count.0.max(4); }

    // Mouse-look and WASD for player 0 (simplified mapping from SDL's per-device assignment)
    let mut p0 = query.iter_mut().find(|p| p.id == 0).unwrap();

    // Sensitivity scales with window size a bit
    let sens = 0.0025;
    p0.yaw -= mouse_delta.dx * sens;
    p0.pitch -= mouse_delta.dy * sens;
    // Clamp pitch similar to original (~ +/- 1.6 rad)
    let clamp = 1.6f32;
    p0.pitch = p0.pitch.clamp(-clamp, clamp);
    // reset mouse delta each frame
    mouse_delta.dx = 0.0;
    mouse_delta.dy = 0.0;

    // Additional players can be controlled via arrow keys, IJKL, and numpad as a convenience
    let sets: [(KeyCode, KeyCode, KeyCode, KeyCode, KeyCode); 4] = [
        (KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD, KeyCode::Space),
        (KeyCode::ArrowUp, KeyCode::ArrowLeft, KeyCode::ArrowDown, KeyCode::ArrowRight, KeyCode::Numpad0),
        (KeyCode::KeyI, KeyCode::KeyJ, KeyCode::KeyK, KeyCode::KeyL, KeyCode::ShiftRight),
        (KeyCode::Numpad8, KeyCode::Numpad4, KeyCode::Numpad5, KeyCode::Numpad6, KeyCode::NumpadEnter),
    ];

    for mut p in &mut query {
        let (up, left, down, right, jump) = sets[p.id.min(3)];
        let forward = kb.pressed(up) as u8;
        let left_b = kb.pressed(left) as u8;
        let back = kb.pressed(down) as u8;
        let right_b = kb.pressed(right) as u8;
        let jump_b = kb.pressed(jump) as u8;
        // Store into a temporary field via Commands? We'll handle velocity in physics directly by querying keyboard again.
        // No need to store a bitmask on the component; physics will re-check pressed state each frame.
    }
}

fn update_physics(
    time: Res<Time>,
    kb: Res<ButtonInput<KeyCode>>,
    mut q: Query<(&mut Transform, &mut Velocity, &mut Player)>,
) {
    let dt = time.delta_secs().max(1e-6);
    let drag = (-dt * DRAG_RATE).exp();
    let diff = 1.0 - drag;

    for (mut transform, mut vel, mut player) in &mut q {
        // Direction from WASD for the player's mapped keys
        let (up, left, down, right, jump) = match player.id {
            0 => (KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD, KeyCode::Space),
            1 => (KeyCode::ArrowUp, KeyCode::ArrowLeft, KeyCode::ArrowDown, KeyCode::ArrowRight, KeyCode::Numpad0),
            2 => (KeyCode::KeyI, KeyCode::KeyJ, KeyCode::KeyK, KeyCode::KeyL, KeyCode::ShiftRight),
            _ => (KeyCode::Numpad8, KeyCode::Numpad4, KeyCode::Numpad5, KeyCode::Numpad6, KeyCode::NumpadEnter),
        };

        let dir_x = (kb.pressed(right) as i8 - kb.pressed(left) as i8) as f32;
        let dir_z = (kb.pressed(up) as i8 - kb.pressed(down) as i8) as f32;
        let norm = (dir_x * dir_x + dir_z * dir_z).sqrt();
        let (sin_yaw, cos_yaw) = player.yaw.sin_cos();
        let acc_x = MOVE_MULT
            * if norm == 0.0 { 0.0 } else { (cos_yaw * dir_x + sin_yaw * dir_z) / norm };
        let acc_z = MOVE_MULT
            * if norm == 0.0 { 0.0 } else { (-sin_yaw * dir_x + cos_yaw * dir_z) / norm };

        // Apply drag and gravity
        vel.x -= vel.x * diff;
        vel.z -= vel.z * diff;
        vel.y -= GRAVITY * dt;

        // Apply acceleration
        vel.x += diff * acc_x / DRAG_RATE;
        vel.z += diff * acc_z / DRAG_RATE;

        // Integrate position similar to original
        transform.translation.x += (dt - diff / DRAG_RATE) * acc_x / DRAG_RATE + diff * vel.x / DRAG_RATE;
        transform.translation.y += -0.5 * GRAVITY * dt * dt + vel.y * dt;
        transform.translation.z += (dt - diff / DRAG_RATE) * acc_z / DRAG_RATE + diff * vel.z / DRAG_RATE;

        // Bounds
        let scale = MAP_BOX_SCALE as f32;
        let bound = scale - player.radius;
        let mut pos = transform.translation;
        let mut hit_x = false;
        let mut hit_y = false;
        let mut hit_z = false;
        if pos.x < -bound { pos.x = -bound; hit_x = true; }
        if pos.x >  bound { pos.x =  bound; hit_x = true; }
        if pos.y <  player.height - scale { pos.y = player.height - scale; hit_y = true; }
        if pos.y >  bound { pos.y =  bound; hit_y = true; }
        if pos.z < -bound { pos.z = -bound; hit_z = true; }
        if pos.z >  bound { pos.z =  bound; hit_z = true; }
        if hit_x { vel.x = 0.0; }
        if hit_z { vel.z = 0.0; }
        if hit_y {
            vel.y = if kb.pressed(jump) { JUMP_VELOCITY } else { 0.0 };
        }
        transform.translation = pos;
    }
}

fn handle_shooting(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut q_players: Query<(&mut Transform, &mut Player)>,
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) { return; }

    // Shooter is player 0
    let (origin, yaw, pitch, shooter_id) = {
        let (shooter_tf, shooter) = q_players.iter().find(|(_, p)| p.id == 0).unwrap();
        (shooter_tf.translation, shooter.yaw, shooter.pitch, shooter.id)
    };

    // Direction from yaw/pitch
    let (sin_yaw, cos_yaw) = yaw.sin_cos();
    let (sin_pitch, cos_pitch) = pitch.sin_cos();
    let dir = Vec3::new(-sin_yaw * cos_pitch, sin_pitch, -cos_yaw * cos_pitch).normalize();

    // Check targets
    for (mut tf, mut target) in &mut q_players {
        if target.id == shooter_id { continue; }
        let offset = tf.translation - origin;
        // Two spheres: feet and head
        let mut hit_count = 0;
        for j in 0..2 {
            let dy = offset.y + if j == 0 { 0.0 } else { target.radius - target.height };
            let d = Vec3::new(offset.x, dy, offset.z);
            let vd = dir.dot(d);
            if vd < 0.0 { continue; }
            let dd = d.length_squared();
            let rr = target.radius * target.radius;
            let vv = 1.0; // dir is normalized
            if vd * vd >= vv * (dd - rr) { hit_count += 1; }
        }
        if hit_count > 0 {
            // Reposition randomly within bounds
            let scale = MAP_BOX_SCALE as f32;
            let mut rng = rand::thread_rng();
            tf.translation.x = scale * (rng.r#gen::<f32>() - 0.5);
            tf.translation.y = scale * (rng.r#gen::<f32>() - 0.5);
            tf.translation.z = scale * (rng.r#gen::<f32>() - 0.5);
        }
    }
}

fn draw_world_gizmos(
    edges: Res<Edges>,
    q_players: Query<(&Transform, &Player)>,
    mut gizmos: Gizmos,
) {
    // Draw edges
    let edge_color = Color::srgb(0.25, 0.25, 0.25);
    for (a, b) in &edges.0 {
        gizmos.line(*a, *b, edge_color);
    }

    // Draw players as simple capsules (vertical)
    for (tf, p) in &q_players {
        // Feet center and head center positions
        let feet = tf.translation + Vec3::Y * (p.radius - p.height);
        let head = tf.translation;
        gizmos.line(feet, head, p.color);
        gizmos.sphere(feet, p.radius, p.color);
        gizmos.sphere(head, p.radius, p.color);
    }
}

fn update_camera_transforms(
    q_players: Query<(&Transform, &Player)>,
    mut q_cams: Query<(&mut Transform, &PlayerCamera), (With<Camera>, Without<Player>)>,
) {
    for (mut cam_tf, cam) in &mut q_cams {
        if let Some((player_tf, player)) = q_players.iter().find(|(_, p)| p.id == cam.player_id) {
            // Camera at player's position + slight height, looking along yaw/pitch
            let cam_pos = player_tf.translation + Vec3::Y * 0.5;
            let (sin_yaw, cos_yaw) = player.yaw.sin_cos();
            let (sin_pitch, cos_pitch) = player.pitch.sin_cos();
            let dir = Vec3::new(-sin_yaw * cos_pitch, sin_pitch, -cos_yaw * cos_pitch);
            *cam_tf = Transform::from_translation(cam_pos).looking_at(cam_pos + dir, Vec3::Y);
        }
    }
}

fn update_camera_viewports(
    windows: Query<&Window>,
    mut q_cams: Query<(&mut Camera, &PlayerCamera)>,
    player_count: Res<PlayerCount>,
) {
    let window = windows.single().expect("primary window");
    let (w, h) = (window.resolution.physical_width(), window.resolution.physical_height());
    let (part_hor, part_ver): (usize, usize) = if player_count.0 > 2 { (2, 2) } else if player_count.0 > 1 { (2, 1) } else { (1, 1) };
    let size_w: u32 = w / part_hor as u32;
    let size_h: u32 = h / part_ver as u32;

    for (mut cam, cam_tag) in &mut q_cams {
        let i = cam_tag.player_id;
        let active = i < player_count.0;
        cam.is_active = active;
        if !active { cam.viewport = None; continue; }
        let mod_x = (i % part_hor) as u32;
        let mod_y = (i / part_hor) as u32;
        cam.viewport = Some(bevy::render::camera::Viewport {
            physical_position: UVec2::new(mod_x * size_w, mod_y * size_h),
            physical_size: UVec2::new(size_w, size_h),
            depth: 0.0..1.0,
        });
    }
}

fn draw_crosshair_gizmos(mut gizmos: Gizmos, cams: Query<(&Transform, &Camera)>) {
    for (tf, cam) in &cams {
        if !cam.is_active { continue; }
        let origin = tf.translation + tf.forward() * 2.0;
        let right = tf.right() * CROSS_WORLD_HALF;
        let up = tf.up() * CROSS_WORLD_HALF;
        gizmos.line(origin - right, origin + right, Color::WHITE);
        gizmos.line(origin - up, origin + up, Color::WHITE);
    }
}

// --- Helpers ---

fn init_edges(scale: i32) -> Edges {
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

