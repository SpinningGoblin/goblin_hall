use bevy::{
    prelude::{
        App, ImagePlugin, IntoSystemAppConfig, IntoSystemAppConfigs, IntoSystemConfig,
        IntoSystemConfigs, IntoSystemSetConfig, OnEnter, OnExit, OnUpdate, PluginGroup,
    },
    window::close_on_esc,
    DefaultPlugins,
};
use events::PointVisited;
use resources::sprites::Handles;
use sets::{Sets, StartupSets};
use state::AppState;
use systems::textures;

mod components;
mod events;
mod resources;
mod sets;
mod state;
mod systems;
mod utils;

fn main() {
    let game_config = resources::config::load_game_configuration().unwrap();

    let mut app = App::new();
    app.init_resource::<Handles>()
        .insert_resource(game_config.world_timer())
        .insert_resource(game_config)
        .add_event::<PointVisited>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .configure_set(Sets::CharacterJobs.after(Sets::Tick))
        .configure_set(Sets::Finishing.after(Sets::CharacterJobs))
        .configure_set(Sets::InputResponse.after(Sets::Input));

    let input_set = (
        systems::camera::process_movement_input,
        systems::camera::zoom_camera,
        systems::targets::move_target,
        systems::targets::swap_targets,
    )
        .in_set(OnUpdate(AppState::InGame))
        .in_set(Sets::Input);

    let input_responses = (systems::zones::place_zone, systems::camera::move_camera)
        .in_set(OnUpdate(AppState::InGame))
        .in_set(Sets::InputResponse);

    let tick_set = (systems::world::tick_game_world)
        .in_set(OnUpdate(AppState::InGame))
        .in_set(Sets::Tick);

    let assign_job_set = (
        systems::jobs::assign_miner_priority,
        systems::jobs::assign_builder_priority,
        systems::jobs::assign_explorer_priority,
        systems::jobs::assign_gatherer_priority,
        systems::jobs::assign_job
            .after(systems::jobs::assign_miner_priority)
            .after(systems::jobs::assign_builder_priority)
            .after(systems::jobs::assign_explorer_priority)
            .after(systems::jobs::assign_gatherer_priority),
    )
        .in_set(Sets::CharacterJobs)
        .in_set(OnUpdate(AppState::InGame));

    let character_job_set = (
        systems::tasks::assign_explorer_task,
        systems::tasks::assign_miner_task,
        systems::tasks::assign_builder_task,
        systems::tasks::assign_gatherer_task,
        systems::tasks::do_walk_work.run_if(systems::world::tick_just_finished),
        systems::tasks::do_mining_work.run_if(systems::world::tick_just_finished),
        systems::tasks::do_clear_exploration_work.run_if(systems::world::tick_just_finished),
        systems::tasks::do_setup_storage_work.run_if(systems::world::tick_just_finished),
        systems::tasks::do_gather_work.run_if(systems::world::tick_just_finished),
        systems::tasks::do_empty_resources_work.run_if(systems::world::tick_just_finished),
    )
        .in_set(Sets::CharacterTasks)
        .after(Sets::CharacterJobs)
        .in_set(OnUpdate(AppState::InGame));

    let finishing_set = (
        systems::characters::show_in_visible_area,
        systems::spawns::characters,
        systems::spawns::structures,
        systems::spawns::map,
        systems::characters::collect_visited_points,
        systems::spawns::clear
            .after(systems::spawns::characters)
            .after(systems::spawns::map),
    )
        .in_set(OnUpdate(AppState::InGame))
        .after(Sets::CharacterTasks)
        .in_set(Sets::Finishing);

    let starting_spawns = (
        systems::targets::spawn,
        systems::init::spawn_starting,
        systems::camera::spawn_camera,
    )
        .after(StartupSets::TextureAtlas)
        .in_schedule(OnEnter(AppState::InGame));

    app.add_state::<AppState>()
        .add_system(close_on_esc)
        .add_systems(input_set)
        .add_systems(input_responses)
        .add_system(tick_set)
        .add_systems(assign_job_set)
        .add_systems(character_job_set)
        .add_systems(finishing_set)
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
