pub mod constants;
pub mod components;
pub mod helpers;
pub mod systems;

use bevy::prelude::*;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Example splitscreen shooter game (Bevy)".into(),
                resolution: (constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(components::PlayerCount(1))
        .insert_resource(components::MouseDelta::default())
        .insert_resource(helpers::init_edges(constants::MAP_BOX_SCALE))
        .insert_resource(components::GameMode(components::AppMode::Playing))
        .insert_resource(components::SettingsRes {
            sensitivity: 0.0025,
            crosshair_half: constants::CROSS_WORLD_HALF,
        })
        .add_systems(Startup, (systems::setup_players, systems::setup_cameras))
        .add_systems(
            Update,
            (
                systems::esc_menu_toggle,
                systems::accumulate_mouse_motion,
                systems::handle_input,
                systems::handle_settings_input,
                systems::update_physics,
                systems::handle_shooting,
                systems::draw_world_gizmos,
                systems::draw_crosshair_gizmos,
                systems::update_camera_transforms,
                systems::update_camera_viewports,
                systems::update_cursor_visibility,
            ),
        )
        .run();
}
