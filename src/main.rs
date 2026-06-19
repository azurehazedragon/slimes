use bevy::{
    camera::Viewport, 
    color::palettes::{
        basic::WHITE, 
        css::{GREEN, RED},
    },
    math::ops::powf,
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, controls)
        .run();
}

fn controls(
    camera_query: Single<(&mut Camera, &mut Transform, &mut Projection)>,
    window: Single<&Window>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
    ) {

    let (mut camera, mut transform, mut projection) = camera_query.into_inner();

    let fspeed = 600.0 * time.delta_secs();
    let window_size = window.resolution.physical_size();

    // Camera zoom controls
    if let Projection::Orthographic(projection2d) = &mut *projection {
        if input.pressed(KeyCode::ShiftLeft) && input.pressed(KeyCode::KeyS) {
            projection2d.scale *= powf(4.0f32, time.delta_secs());
        }

        if input.pressed(KeyCode::ShiftLeft) && input.pressed(KeyCode::KeyW) {
            projection2d.scale *= powf(0.25f32, time.delta_secs());
        }
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
        if viewport.physical_size.x != window_size.x || viewport.physical_size.y != window_size.y {
            viewport.physical_size = window_size.as_vec2().as_uvec2();
        }
    }
}

fn setup(
    mut commands: Commands, 
    window: Single<&Window>,
    asset_server: Res<AssetServer>) {

    let window_size = window.resolution.physical_size().as_vec2();

    commands.spawn((
            Camera2d,
            Camera {
                viewport: Some(Viewport {
                    physical_size: window_size.as_uvec2(),
                    ..default()
                }),
                ..default()
            },
    ));

    commands.spawn(
        Sprite::from_image(
            asset_server.load("Grass1.png"),
    ));
    
    commands.spawn((
            Sprite::from_image(asset_server.load("LightGrass1.png")),
            Transform::from_xyz(54.0,35.0,0.)
    ));
}
