use bevy::{
    prelude::{App, ImagePlugin, IntoSystemDescriptor, PluginGroup, SystemSet},
    window::close_on_esc,
    DefaultPlugins,
};
use labels::Label;
use resources::sprites::Handles;
use state::AppState;
use systems::{startup, textures};

mod components;
mod labels;
mod resources;
mod state;
mod systems;

fn main() {
    let game_config = resources::config::load_game_configuration().unwrap();

    App::new()
        .init_resource::<Handles>()
        .insert_resource(game_config)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state(AppState::Startup)
        .add_system(close_on_esc)
        .add_system_set(SystemSet::on_enter(AppState::Startup).with_system(textures::load))
        .add_system_set(SystemSet::on_update(AppState::Startup).with_system(textures::check))
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(startup)
                .with_system(systems::camera::spawn_camera),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(
                    systems::camera::process_movement_input.label(Label::CameraMovementInput),
                )
                .with_system(
                    systems::camera::move_camera
                        .label(Label::CameraMovement)
                        .after(Label::CameraMovementInput),
                )
                .with_system(systems::camera::zoom_camera)
                .with_system(systems::mouse::move_mouse_target),
        )
        .run();
}
