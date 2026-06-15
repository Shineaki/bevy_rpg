use avian2d::{math::*, parry::math::Vector2, prelude::*};
use bevy::{ecs::query::Has, prelude::*};

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MovementAction>().add_systems(
            Update,
            (
                keyboard_input,
                movement,
                apply_movement_damping,
            )
                .chain(),
        );
    }
}

/// A [`Message`] written for a movement input action.
#[derive(Message)]
pub enum MovementAction {
    Move(Vector2),
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    body: RigidBody,
    collider: Collider,
    locked_axes: LockedAxes,
    movement: MovementBundle,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
}

impl MovementBundle {
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(50.0, 0.9)
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            body: RigidBody::Dynamic,
            collider,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping);
        self
    }
}

/// Sends [`MovementAction`] events based on keyboard input.
fn keyboard_input(
    mut movement_writer: MessageWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;
    let direction = Vector2::new(horizontal as Scalar, vertical as Scalar).clamp_length_max(1.);

    if direction != Vector2::ZERO {
        movement_writer.write(MovementAction::Move(direction));
    }

    // if keyboard_input.just_pressed(KeyCode::Space) {
    //     movement_writer.write(MovementAction::Jump);
    // }
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn movement(
    time: Res<Time>,
    mut movement_reader: MessageReader<MovementAction>,
    mut controllers: Query<(
        &MovementAcceleration,
        &mut LinearVelocity,
    )>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_secs_f64().adjust_precision();

    for event in movement_reader.read() {
        for (movement_acceleration, mut linear_velocity) in
            &mut controllers
        {
            match event {
                MovementAction::Move(direction) => {
                    // let dir: Vec2 = direction.normalize_or_zero();
                    linear_velocity.0 += movement_acceleration.0 * direction * delta_time;
                }
                // MovementAction::Jump => {
                //     if is_grounded {
                //         linear_velocity.y = jump_impulse.0;
                //     }
                // }
            }
        }
    }
}

/// Slows down movement in the X direction.
fn apply_movement_damping(
    time: Res<Time>,
    mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_secs_f64().adjust_precision();

    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= 1.0 / (1.0 + damping_factor.0 * delta_time);
        linear_velocity.y *= 1.0 / (1.0 + damping_factor.0 * delta_time);
    }
}