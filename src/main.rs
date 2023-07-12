use bevy::{
    prelude::{
        in_state, App, ImagePlugin, IntoSystemConfigs, IntoSystemSetConfig, OnEnter, OnExit,
        PluginGroup, Update,
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
        .configure_set(Update, Sets::CharacterJobs.after(Sets::Tick))
        .configure_set(Update, Sets::Finishing.after(Sets::CharacterJobs))
        .configure_set(Update, Sets::InputResponse.after(Sets::Input));

    let input_set = (
        systems::camera::process_movement_input,
        systems::camera::zoom_camera,
        systems::targets::move_target,
        systems::targets::swap_targets,
        systems::jobs::swap_job_assignment_mode,
        systems::world::pause_world_tick,
    )
        .run_if(in_state(AppState::InGame))
        .in_set(Sets::Input);

    let input_responses = (
        systems::zones::place_zone,
        systems::camera::move_camera,
        systems::jobs::manually_assign_job,
    )
        .after(Sets::Input)
        .run_if(in_state(AppState::InGame))
        .in_set(Sets::InputResponse);

    let tick_set = (systems::world::tick_game_world).in_set(Sets::Tick);

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
        .run_if(in_state(AppState::InGame))
        .after(Sets::InputResponse);

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
        .run_if(in_state(AppState::InGame))
        .after(Sets::CharacterJobs);

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
        .after(Sets::CharacterTasks)
        .run_if(in_state(AppState::InGame))
        .in_set(Sets::Finishing);

    let starting_spawns = (
        systems::targets::spawn,
        systems::init::spawn_starting,
        systems::camera::spawn_camera,
    )
        .after(StartupSets::TextureAtlas);

    app.add_state::<AppState>()
        .add_systems(Update, close_on_esc)
        .add_systems(Update, input_set)
        .add_systems(Update, input_responses)
        .add_systems(Update, tick_set)
        .add_systems(Update, assign_job_set)
        .add_systems(Update, character_job_set)
        .add_systems(Update, finishing_set)
        .add_systems(OnEnter(AppState::Startup), textures::load)
        .add_systems(Update, textures::check.run_if(in_state(AppState::Startup)))
        .add_systems(
            OnExit(AppState::Startup),
            textures::finalize_texture_atlas.in_set(StartupSets::TextureAtlas),
        )
        .add_systems(
            OnEnter(AppState::InGame),
            starting_spawns.after(StartupSets::TextureAtlas),
        )
        .run();
}
