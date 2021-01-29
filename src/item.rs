const ITEM_SIZE: f32 = 50.0;

use std::time::Duration;

use bevy::prelude::*;

use crate::{velocity::Velocity, Materials};

pub struct Item;

pub struct ItemSpawnTimer(Timer);
impl Default for ItemSpawnTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(1000), true))
    }
}

pub fn spawn_item(
    commands: &mut Commands,
    materials: Res<Materials>,
    time: Res<Time>,
    mut timer: Local<ItemSpawnTimer>,
    windows: Res<Windows>,
) {
    if timer.0.tick(time.delta_seconds()).finished() {
        let window = windows.get_primary().unwrap();
        let x = (rand::random::<f32>() * window.width()) - (window.width() / 2.0);
        let y = window.height() / 2.0 + ITEM_SIZE / 2.0;
        commands
            .spawn(SpriteBundle {
                material: materials.item_material.clone(),
                sprite: Sprite::new(Vec2::new(ITEM_SIZE, ITEM_SIZE)),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..Default::default()
            })
            .with(Item)
            .with(Velocity(Vec3::new(0.0, -1.0, 0.0) * 150.0));
    }
}

/// despawn item if item is out of window.
/// FIXME: maybe not need if score system is made.
pub fn despawn_item(
    commands: &mut Commands,
    mut items: Query<(Entity, &Transform), With<Item>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    for (entity, transform) in items.iter_mut() {
        if transform.translation.y < 0.0 - window.height() / 2.0 - ITEM_SIZE / 2.0 {
            commands.despawn(entity);
        }
    }
}

pub fn accelerate_item(mut velocities: Query<&mut Velocity, With<Item>>, time: Res<Time>) {
    let delta_seconds = f32::min(0.2, time.delta_seconds());
    for mut velocity in velocities.iter_mut() {
        velocity.0 += Vec3::new(0.0, -1.0, 0.0) * 750.0 * delta_seconds;
    }
}
