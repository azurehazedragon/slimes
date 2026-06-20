use bevy::{
    asset::RenderAssetUsages,
    camera::Viewport,
    color::palettes::basic::PURPLE,
    input::mouse::AccumulatedMouseScroll,
    math::ops::powf,
    mesh::Indices,
    prelude::Mesh,
    prelude::*,
    render::{
        RenderPlugin, render_resource::PrimitiveTopology, render_resource::WgpuFeatures,
        settings::WgpuSettings,
    },
};

use hexx::{Hex, HexLayout, MeshInfo, PlaneMeshBuilder};
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, controls)
        .add_systems(Update, animate_sprite)
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() 
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn hexagonal_mesh(mesh_info: MeshInfo) -> Mesh {
    let vertices: Vec<[f32; 3]> = mesh_info
        .vertices
        .into_iter()
        .map(|v| [v.x, v.z, v.y])
        .collect();

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}

fn controls(
    camera_query: Single<(&mut Camera, &mut Transform, &mut Projection)>,
    window: Single<&Window>,
    input: Res<ButtonInput<KeyCode>>,
    wheel: Res<AccumulatedMouseScroll>,
    time: Res<Time<Fixed>>,
) {
    let (mut camera, mut transform, mut projection) = camera_query.into_inner();

    if let Projection::Orthographic(projection2d) = &mut *projection {
        let fspeed = 200.0 * time.delta_secs() * projection2d.scale;
        let window_size = window.resolution.physical_size();

        // Camera zoom controls
        if wheel.delta.y < 0. {
            projection2d.scale *= powf(4.0f32, time.delta_secs() * 5.);
        }

        if wheel.delta.y > 0. {
            projection2d.scale *= powf(0.25f32, time.delta_secs() * 5.);
        }

        // Camera movement controls
        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += fspeed;
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= fspeed;
        }
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= fspeed;
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation.x += fspeed;
        }

        if let Some(viewport) = camera.viewport.as_mut() {
            // Reset viewport size on window resize
            if viewport.physical_size.x != window_size.x
                || viewport.physical_size.y != window_size.y
            {
                viewport.physical_size = window_size.as_vec2().as_uvec2();
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("slime_spritesheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 9, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 5 };

    let grass_list = [
        "Grass1.png",
        "Grass2.png",
        "Grass3.png",
        "Grass4.png",
        "Grass5.png",
        "LightGrass1.png",
        "LightGrass2.png",
        "LightGrass3.png",
        "LightGrass4.png",
        "LightGrass5.png",
    ];

    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport { ..default() }),
            ..default()
        },
    ));

   
    let layout = HexLayout {
        scale: Vec2::splat(128.),
        orientation: hexx::HexOrientation::Flat,
        ..default()
    };

    let hex_meshes: Vec<MeshInfo> = Hex::ZERO
        .range(10)
        .map(|hex| PlaneMeshBuilder::new(&layout).at(hex).build())
        .collect();

    for (i, mesh_info) in hex_meshes.iter().enumerate() {
        println!("Hex {i}: first vertex = {:?}", mesh_info.vertices.first());
    }

    let mut rng = rand::rng();

    for mesh_info in hex_meshes {
        commands.spawn((
            Mesh2d(meshes.add(hexagonal_mesh(mesh_info))),
            MeshMaterial2d(materials.add(asset_server.load(*grass_list
                        .choose(&mut rng)
                        .unwrap()
                        ))),
        ));
    }

     commands.spawn((Sprite {
                 image: texture,
                 texture_atlas: Some(TextureAtlas {
                     layout: texture_atlas_layout,
                     index: animation_indices.first,
                 }),
                 color: Color::srgba(0.7, 0.1, 0.2, 0.85),
                 custom_size: Some(Vec2::splat(128.)),
                 ..default()
            },
        Transform::from_xyz(0., 0., 1.0),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
     ));

}
