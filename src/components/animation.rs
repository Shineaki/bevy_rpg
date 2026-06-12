use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    Idle,
    Walking,
}

#[derive(Component)]
pub struct AnimationIndices {
    pub idle: (usize, usize),      // (first, last) for idle
    pub walking: (usize, usize),   // (first, last) for walking
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl AnimationIndices {
    pub fn get(&self, state: AnimationState) -> (usize, usize) {
        match state {
            AnimationState::Idle => self.idle,
            AnimationState::Walking => self.walking,
        }
    }
}