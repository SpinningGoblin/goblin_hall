use bevy::prelude::{AssetServer, Res, ResMut};

use crate::resources::sprites::Handles;

pub fn load(mut handles: ResMut<Handles>, asset_server: Res<AssetServer>) {
    handles.all = asset_server.load_folder("sprites").unwrap();
}
