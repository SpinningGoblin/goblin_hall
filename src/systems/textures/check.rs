use bevy::{
    asset::LoadState,
    prelude::{AssetServer, Res, ResMut, State},
};

use crate::{resources::sprites::Handles, state::AppState};

pub fn check(
    mut state: ResMut<State<AppState>>,
    handles: ResMut<Handles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(handles.all.iter().map(|handle| handle.id))
    {
        state.set(AppState::InGame).unwrap();
    }
}
