use hexx::*;

use bevy::prelude::*;

pub fn setup_world_layout (
    mut commands: Commands,
){
    commands.insert_resource(WorldHexLayout {layout: HexLayout {
        scale: Vec2::splat(128.),
        orientation: hexx::HexOrientation::Flat,
        ..default()
    }});
}

#[derive(Resource)]
pub struct WorldHexLayout {
    pub layout: HexLayout
}

#[derive(Component)]
pub struct HexPosition(pub Hex);

impl HexPosition {
pub fn get_world_pos(&self, hex_layout: &HexLayout) -> Vec2 {
        hex_layout.hex_to_world_pos(self.0)
    }
}

