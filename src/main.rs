use bevy::{
    prelude::{App, ImagePlugin, IntoSystemDescriptor, PluginGroup, SystemSet},
    window::close_on_esc,
    DefaultPlugins,
};
use labels::{Labels, StartupLabels};
use resources::sprites::Handles;
use state::AppState;
use systems::textures;

mod components;
mod labels;
mod resources;
mod state;
mod systems;

fn main() {
    let game_config = resources::config::load_game_configuration().unwrap();

    let mut app = App::new();
    app.init_resource::<Handles>()
        .insert_resource(game_config.world_timer())
        .insert_resource(game_config)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));

    let input_set = SystemSet::on_update(AppState::InGame)
        .label(Labels::Input)
        .with_system(systems::camera::process_movement_input)
        .with_system(systems::camera::zoom_camera)
        .with_system(systems::targets::move_target);

    let input_responses = SystemSet::on_update(AppState::InGame)
        .label(Labels::InputResponse)
        .after(Labels::Input)
        .with_system(systems::zones::place_zone)
        .with_system(systems::camera::move_camera);

    let tick_set = SystemSet::new()
        .label(Labels::Tick)
        .with_system(systems::world::tick_game_world);

    let character_job_set = SystemSet::on_update(AppState::InGame)
        .label(Labels::CharacterJobs)
        .after(Labels::Tick)
        .with_system(systems::jobs::assign_job)
        .with_system(systems::tasks::build_todo)
        .with_system(systems::tasks::do_task_work)
        .with_system(systems::tasks::remove_todo);

    let character_cleanup_set = SystemSet::on_update(AppState::InGame)
        .label(Labels::CharacterCleanup)
        .after(Labels::CharacterJobs)
        .with_system(systems::characters::show_in_visible_area);

    app.add_state(AppState::Startup)
        .add_system(close_on_esc)
        .add_system_set(SystemSet::on_enter(AppState::Startup).with_system(textures::load))
        .add_system_set(SystemSet::on_update(AppState::Startup).with_system(textures::check))
        .add_system_set(SystemSet::on_exit(AppState::Startup).with_system(
            systems::textures::finalize_texture_atlas.label(StartupLabels::TextureAtlas),
        ))
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(systems::targets::spawn.after(StartupLabels::TextureAtlas))
                .with_system(systems::map::spawn_starting.after(StartupLabels::TextureAtlas))
                .with_system(systems::camera::spawn_camera),
        )
        .add_system_set(input_set)
        .add_system_set(input_responses)
        .add_system_set(tick_set)
        .add_system_set(character_job_set)
        .add_system_set(character_cleanup_set)
        .run();
}
