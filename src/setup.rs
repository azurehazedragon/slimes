use bevy::{
    camera::Viewport, 
    prelude::{Mesh, *}, 
};

use hexx::*;
use crate::{
    hex::{get_hex_tex, hexagonal_mesh},
    world::{WorldHexLayout, HexPosition},
    slime::{AnimationTimer, MoveState},
};

use rand::*;

const HEX_SIZE: u32 = 2;
const WORLD_SIZE: u32 = 16; 

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    layout: Res<WorldHexLayout>,
) {
    let slime_spritesheet = asset_server.load("slime/slime_spritesheet.png");
    let hex_tilesheet_grass = asset_server.load("hex/terrain_grass.png");
    let slime_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 9, 1, None, None);
    let hex_layout = TextureAtlasLayout::from_grid(UVec2::splat(125), 5, 5, Some(UVec2{x: 5, y: 5}), None);
    let slime_atlas_layout = texture_atlas_layouts.add(slime_layout);

    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport { ..default() }),
            ..default()
        },
    ));

    let mut rng = rand::rng();

    let world_grid = Hex::ZERO.range(WORLD_SIZE);
    
    let hex_material = materials.add(hex_tilesheet_grass);

    for chunk in world_grid {
        let center = chunk.to_higher_res(HEX_SIZE);
        let children = center.range(HEX_SIZE);

        let hex_chunk_mesh = children
            .map(|hex| {
                PlaneMeshBuilder::new(&layout.layout)
                    .at(hex)
                    .with_uv_options(get_hex_tex(&hex_layout, rng.random_range(0..3)))
                    .build()
            })
            .reduce(|mut acc, mesh| {
                acc.merge_with(mesh);
                acc
            })
            .unwrap();

        commands.spawn((
            Mesh2d(meshes.add(hexagonal_mesh(hex_chunk_mesh))),
            MeshMaterial2d(hex_material.clone()),
        ));
    }

    for _ in 0..5 {
    commands.spawn((
        Sprite {
            image: slime_spritesheet.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: slime_atlas_layout.clone(),
                index: 0,
            }),
            color: Color::srgba(f32::from(rng.random_bool(0.5)), f32::from(rng.random_bool(0.5)), f32::from(rng.random_bool(0.5)), 0.85),
            custom_size: Some(Vec2::splat(200.)),
            ..default()
        },
        HexPosition(Hex::ZERO),
        Transform::default(),
        MoveState::Idle,
        AnimationTimer {timer: Timer::from_seconds(0.01, TimerMode::Repeating), step_size: 0., distance: 0., direction: vec2(0., 0.)},
    ));
    }
}
