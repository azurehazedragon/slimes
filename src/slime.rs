use bevy::prelude::*;
use rand::*;
use hexx::*;

use crate::world::{WorldHexLayout, HexPosition, get_new_hex_direction}; 

pub struct SlimePlugin;

impl Plugin for SlimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_slimes)
            .add_systems(Update, 
                (move_slimes,
                animate_sprite)
            );
    }
}

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    // Number of steps it will take to reach the destination
    pub distance: f32,
    // Amount to move each step
    pub step_size: f32,
    // Directino to move in to get to the destination
    pub direction: Vec2,
}

#[derive(Component, PartialEq)]
pub enum MoveState {
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

pub fn move_slimes(
    time: Res<Time>,
    world_layout: Res<WorldHexLayout>,
    mut query: Query<(&mut Transform, &mut HexPosition, &mut MoveState, &mut AnimationTimer)>
) {

    for(mut transform, mut hex_position, mut move_state, mut animation_timer) in &mut query {
        if *move_state == MoveState::Jump {
            let origin = hex_position.get_world_pos(&world_layout.layout);
            hex_position.0 = hex_position.0.neighbor(get_new_hex_direction());
           let dest = hex_position.get_world_pos(&world_layout.layout);

            animation_timer.distance = 0.3;
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

pub fn animate_sprite(
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

pub fn setup_slimes(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    ) {

    info!("Setting up slimes");

    let slime_spritesheet = asset_server.load("slime/slime_spritesheet.png");
    let slime_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 9, 1, None, None);
    let slime_atlas_layout = texture_atlas_layouts.add(slime_layout);

    let mut rng = rand::rng();

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
        AnimationTimer {timer: Timer::from_seconds(0.2, TimerMode::Repeating), step_size: 0., distance: 0., direction: vec2(0., 0.)},
    ));
    }
}
