use bevy::prelude::*;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct MyWorldCoords(pub Vec2);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Player;