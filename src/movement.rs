use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use super::components::*;

// Updating the position and velocity of game objects
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // Only when actively playing
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(paddle_movement_system.system())
                    .with_system(ball_collision_system.system())
                    .with_system(ball_movement_system.system())
            );
    }
}

fn paddle_movement_system(
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    if let Ok((paddle, mut transform)) = query.single_mut() {
        let mut direction = 0.0;
        if key_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }

        if key_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        let translation = &mut transform.translation;
        // Move the paddle horizontally
        translation.x += time.delta_seconds() * direction * paddle.speed;
        // Bound the paddle within the walls
        translation.x = translation.x.min(220.0).max(-220.0);
    }
}

fn ball_movement_system(
    time: Res<Time>, 
    mut ball_query: Query<(&Ball, &mut Transform)>
) {
    // Clamp the timestep to stop the ball from escaping when the game starts
    let delta_seconds = f32::min(0.2, time.delta_seconds());

    if let Ok((ball, mut transform)) = ball_query.single_mut() {
        transform.translation += ball.velocity * delta_seconds;
    }
}

fn ball_collision_system(
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    collider_query: Query<(&Collider, &Transform, &Sprite)>,
) {
    if let Ok((mut ball, ball_transform, sprite)) = ball_query.single_mut() {
        let ball_size = sprite.size;
        let velocity = &mut ball.velocity;

        // Check collision with walls
        for (collider, transform, sprite) in collider_query.iter() {
            let collision = collide(
                ball_transform.translation,
                ball_size,
                transform.translation,
                sprite.size,
            );
            if let Some(collision) = collision {
                // Increment the scoreboard if ball collides with paddle (hit)
                // Decrement if it collides with the bottom wall (miss)
                if let Collider::Paddle = *collider {
                    scoreboard.score += 1;
                } else if let Collider::Bottom = *collider {
                    scoreboard.score -= 1;
                }

                // Reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // Only reflect if the ball's velocity is going in the opposite direction of the
                // Collision
                match collision {
                    Collision::Left => reflect_x = velocity.x > 0.0,
                    Collision::Right => reflect_x = velocity.x < 0.0,
                    Collision::Top => reflect_y = velocity.y < 0.0,
                    Collision::Bottom => reflect_y = velocity.y > 0.0,
                }

                // Reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    velocity.x = -velocity.x;
                }

                // Reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    velocity.y = -velocity.y;
                }

                // Break if this collide is on a solid, otherwise continue check whether a solid is
                // Also in collision
                if let Collider::Solid = *collider {
                    break;
                }
            }
        }
    }
}
