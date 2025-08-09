use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub id: usize,
    pub yaw: f32,   // radians
    pub pitch: f32, // radians
    pub radius: f32,
    pub height: f32,
    pub color: Color,
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct PlayerCamera {
    pub player_id: usize,
}

#[derive(Resource)]
pub struct PlayerCount(pub usize);

#[derive(Resource)]
pub struct Edges(pub Vec<(Vec3, Vec3)>);

#[derive(Resource, Default)]
pub struct MouseDelta {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Playing,
    Menu,
    Settings,
}

#[derive(Resource)]
pub struct GameMode(pub AppMode);

#[derive(Resource)]
pub struct SettingsRes {
    pub sensitivity: f32,
    pub crosshair_half: f32,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum CrosshairKind {
    Vertical,
    Horizontal,
}

#[derive(Component)]
pub struct Crosshair {
    pub player_id: usize,
    pub kind: CrosshairKind,
}
