mod setup;
mod input;
mod slime;
mod world;
mod hex;

use bevy::{
    dev_tools::fps_overlay::{
        FpsOverlayConfig, 
        FpsOverlayPlugin, 
        FrameTimeGraphConfig
    },
    text::FontSmoothing,
    input::mouse::AccumulatedMouseScroll, 
    math::{ops::powf}, 
    prelude::*, 
};

use crate::{
    input::controls,
    setup::{setup}, 
    world::{setup_world_layout},
    slime::{animate_sprite, move_slimes},
};


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
                        target_fps: 240.0,
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
