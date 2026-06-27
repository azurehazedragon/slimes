use super::*;

pub fn controls(
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
