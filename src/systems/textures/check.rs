use bevy::{
    asset::LoadState,
    prelude::{AssetServer, NextState, Res, ResMut},
};

use crate::{resources::sprites::Handles, state::AppState};

pub fn check(
    mut next_state: ResMut<NextState<AppState>>,
    handles: ResMut<Handles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(handles.all.iter().map(|handle| handle.id()))
    {
        next_state.set(AppState::InGame);
    }
}
