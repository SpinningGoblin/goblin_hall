use bevy::{
    prelude::{
        App, ImagePlugin, IntoSystemAppConfig, IntoSystemAppConfigs, IntoSystemConfig,
        IntoSystemConfigs, IntoSystemSetConfig, OnEnter, OnExit, OnUpdate, PluginGroup,
    },
    window::close_on_esc,
    DefaultPlugins,
};
use resources::sprites::Handles;
use sets::{Sets, StartupSets};
use state::AppState;
use systems::textures;

mod components;
mod resources;
mod sets;
mod state;
mod systems;

fn main() {
    let game_config = resources::config::load_game_configuration().unwrap();

    let mut app = App::new();
    app.init_resource::<Handles>()
        .insert_resource(game_config.world_timer())
        .insert_resource(game_config)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .configure_set(Sets::CharacterJobs.after(Sets::Tick))
        .configure_set(Sets::CharacterCleanup.after(Sets::CharacterJobs))
        .configure_set(Sets::InputResponse.after(Sets::Input));

    let input_set = (
        systems::camera::process_movement_input,
        systems::camera::zoom_camera,
        systems::targets::move_target,
    )
        .in_set(OnUpdate(AppState::InGame))
        .in_set(Sets::Input);

    let input_responses = (systems::zones::place_zone, systems::camera::move_camera)
        .in_set(OnUpdate(AppState::InGame))
        .in_set(Sets::InputResponse);

    let tick_set = (systems::world::tick_game_world)
        .in_set(OnUpdate(AppState::InGame))
        .in_set(Sets::Tick);

    let character_job_set = (
        systems::jobs::assign_job,
        systems::tasks::build_todo,
        systems::tasks::do_task_work,
        systems::tasks::remove_todo,
    )
        .in_set(Sets::CharacterJobs)
        .in_set(OnUpdate(AppState::InGame));

    let character_cleanup_set = (systems::characters::show_in_visible_area)
        .in_set(OnUpdate(AppState::InGame))
        .in_set(Sets::CharacterCleanup);

    let starting_spawns = (
        systems::targets::spawn,
        systems::map::spawn_starting,
        systems::camera::spawn_camera,
    )
        .after(StartupSets::TextureAtlas)
        .in_schedule(OnEnter(AppState::InGame));

    app.add_state::<AppState>()
        .add_system(close_on_esc)
        .add_systems(input_set)
        .add_systems(input_responses)
        .add_system(tick_set)
        .add_systems(character_job_set)
        .add_system(character_cleanup_set)
        .add_system(textures::load.in_schedule(OnEnter(AppState::Startup)))
        .add_system(textures::check.in_set(OnUpdate(AppState::Startup)))
        .add_system(
            textures::finalize_texture_atlas
                .in_schedule(OnExit(AppState::Startup))
                .in_set(StartupSets::TextureAtlas),
        )
        .add_systems(starting_spawns)
        .run();
}
