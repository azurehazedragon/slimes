use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    text::FontSmoothing,
    asset::RenderAssetUsages, 
    camera::Viewport, 
    input::mouse::AccumulatedMouseScroll, 
    math::{ops::powf}, 
    mesh::Indices, 
    prelude::{Mesh, *}, 
    render::render_resource::PrimitiveTopology,
};

use hexx::*;
use rand::prelude::*;

const HEX_SIZE: u32 = 2;
const WORLD_SIZE: u32 = 16; 

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            FpsOverlayPlugin { 
            config: FpsOverlayConfig {
                    text_config: TextFont {
                        // Here we define size of our overlay
                        font_size: 42.,
                        // If we want, we can use a custom font
                        font: default(),
                        // We could also disable font smoothing,
                        font_smoothing: FontSmoothing::default(),
                        ..default()
                    },
                    // We can also change color of the overlay
                    text_color: Color::srgb(1.0, 1.0, 1.0),
                    // We can also set the refresh interval for the FPS counter
                    refresh_interval: core::time::Duration::from_millis(100),
                    enabled: true,
                    frame_time_graph_config: FrameTimeGraphConfig {
                        enabled: true,
                        // The minimum acceptable fps
                        min_fps: 30.0,
                        // The target fps
                        target_fps: 144.0,
                    },
                },
            },
        ))
        .add_systems(Startup, (setup_world_layout, setup).chain())
        .add_systems(Update, controls)
        .add_systems(Update, animate_sprite)
        .add_systems(Update, move_slimes)
        .run();
}

#[derive(Resource)]
struct WorldHexLayout {
    layout: HexLayout
}

#[derive(Component)]
struct HexPosition(Hex);

impl HexPosition {
    fn get_world_pos(&self, hex_layout: &HexLayout) -> Vec2 {
        hex_layout.hex_to_world_pos(self.0)
    }
}
    

#[derive(Component)]
struct AnimationTimer {
    timer: Timer,
    // Number of steps it will take to reach the destination
    distance: f32,
    // Amount to move each step
    step_size: f32,
    // Directino to move in to get to the destination
    direction: Vec2,
}

#[derive(Component, PartialEq)]
enum MoveState {
    Idle,
    Jump,
    Jumping,
    Moving,
    Land,
}

impl MoveState {
    fn get_sprite_indices(&self) -> (usize, usize) {
        match self {
            MoveState::Idle => (0, 4),
            MoveState::Jump => (5, 5),
            MoveState::Jumping => (7, 7),
            MoveState::Moving => (6, 6),
            MoveState::Land => (8, 8),
        }
    }
}

fn get_new_hex_direction() -> EdgeDirection {
    let mut rng = rand::rng();

    let directions = [ 
        EdgeDirection::FLAT_TOP,
        EdgeDirection::FLAT_TOP_RIGHT,
        EdgeDirection::FLAT_BOTTOM_RIGHT,
        EdgeDirection::FLAT_BOTTOM,
        EdgeDirection::FLAT_BOTTOM_LEFT,
        EdgeDirection::FLAT_TOP_LEFT,
    ];

    directions[rng.random_range(0..directions.len())]
}

fn move_slimes(
    time: Res<Time>,
    world_layout: Res<WorldHexLayout>,
    mut query: Query<(&mut Transform, &mut HexPosition, &mut MoveState, &mut AnimationTimer)>
) {

    for(mut transform, mut hex_position, mut move_state, mut animation_timer) in &mut query {
        if *move_state == MoveState::Jump {
            let origin = hex_position.get_world_pos(&world_layout.layout);
            hex_position.0 = hex_position.0.neighbor(get_new_hex_direction());
           let dest = hex_position.get_world_pos(&world_layout.layout);

            animation_timer.distance = 0.02;
            animation_timer.step_size = origin.distance(dest) / animation_timer.distance;
            animation_timer.direction = (dest - origin).normalize();

            *move_state = MoveState::Moving
        }
        else if *move_state == MoveState::Moving{
                let delta = time.delta().as_secs_f32();
                animation_timer.distance -= delta;
                if animation_timer.step_size != 0. {
                    let v = animation_timer.direction * (animation_timer.step_size * delta);
                    transform.translation += vec3(v.x ,v.y, 0.);
                }

                if animation_timer.distance <= 0. { 
                    *move_state = MoveState::Land;
                }
        }
        else {
            let pos = hex_position.get_world_pos(&world_layout.layout);
            transform.translation = vec3(pos.x, pos.y, 1.);
        }
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut MoveState, &mut AnimationTimer, &mut Sprite)>,
) {
    for (mut move_state, mut timer, mut sprite) in &mut query {
        let mut rng = rand::rng();

        timer.timer.tick(time.delta());

        if timer.timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            let (_, last_index) = move_state.get_sprite_indices();
            atlas.index = if atlas.index == last_index {
                if *move_state == MoveState::Idle {
                    if rng.random_bool(0.9) {
                        *move_state = MoveState::Jump;
                    }
                }
                else if *move_state == MoveState::Jump {
                    *move_state = MoveState::Jumping;
                }
                else if *move_state == MoveState::Jumping {
                    *move_state = MoveState::Moving;
                }
                else if *move_state == MoveState::Land {
                    *move_state = MoveState::Idle;
                }

                let (first_index, _) = move_state.get_sprite_indices();
                first_index
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

fn get_hex_tex(atlas_layout: &TextureAtlasLayout, hex_texure_index: usize) -> UVOptions {
    let rect = atlas_layout.textures[hex_texure_index];
    let (uv_max, uv_min) = (rect.max.as_vec2(), rect.min.as_vec2());
    UVOptions::new().with_rect(
        uv_min / atlas_layout.size.as_vec2(),
        uv_max / atlas_layout.size.as_vec2(),
    ).flip_v()
}

fn setup_world_layout (
    mut commands: Commands,
){
    commands.insert_resource(WorldHexLayout {layout: HexLayout {
        scale: Vec2::splat(128.),
        orientation: hexx::HexOrientation::Flat,
        ..default()
    }});
}

fn setup(
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

    for _i in 0..5 {
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
