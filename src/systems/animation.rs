use crate::components::{AnimationIndices, AnimationState, AnimationTimer, utils::Player};
use bevy::prelude::*;

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut Sprite,
        &AnimationState,
    )>,
) {
    for (indices, mut timer, mut sprite, state) in &mut query {
        timer.tick(time.delta());

        let (first, last) = indices.get(*state);

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == last {
                    first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

pub fn handle_animation_transitions(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&AnimationIndices, &mut Sprite, &mut AnimationTimer, &mut AnimationState), With<Player>>,
) {
    for (indices, mut sprite, mut timer, mut state) in &mut query {
        let c_state = *state;
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::ArrowLeft) {
            *state = AnimationState::Walking;
        } else {
            *state = AnimationState::Idle;
        }
        if *state != c_state {
            timer.reset();
            let idx = indices.get(*state);
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = idx.0
            }
        }
    }
}
