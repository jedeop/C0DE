use bevy::prelude::*;

use crate::{velocity::Velocity, Materials};

pub struct Item;

pub fn spawn_item(commands: &mut Commands, materials: Res<Materials>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    commands
        .spawn(SpriteBundle {
            material: materials.item_material.clone(),
            sprite: Sprite::new(Vec2::new(50.0, 50.0)),
            transform: Transform::from_translation(Vec3::new(0.0, window.height() / 2.0, 0.0)),
            ..Default::default()
        })
        .with(Item)
        .with(Velocity(Vec3::new(0.0, -1.0, 0.0) * 100.0));
}
