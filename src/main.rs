use bevy::{
    prelude::{App, ImagePlugin, PluginGroup, SystemSet},
    DefaultPlugins,
};
use resources::sprites::Handles;
use state::AppState;
use systems::{startup, textures};

mod resources;
mod state;
mod systems;

fn main() {
    App::new()
        .init_resource::<Handles>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state(AppState::Startup)
        .add_system_set(SystemSet::on_enter(AppState::Startup).with_system(textures::load))
        .add_system_set(SystemSet::on_update(AppState::Startup).with_system(textures::check))
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(startup))
        .run();
}
