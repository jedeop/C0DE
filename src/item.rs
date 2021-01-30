const ITEM_SIZE: f32 = 50.0;

use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::{velocity::Velocity, Materials};

pub struct Item {
    is_good: bool,
}

pub struct ItemSpawnTimer(Timer);
impl Default for ItemSpawnTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(750), true))
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

        let is_good = rand::thread_rng().gen_range(0..3) == 0;
        let material = if is_good {
            materials.item_good_material.clone()
        } else {
            materials.item_bad_material.clone()
        };
        commands
            .spawn(SpriteBundle {
                material: material,
                sprite: Sprite::new(Vec2::new(ITEM_SIZE, ITEM_SIZE)),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..Default::default()
            })
            .with(Item { is_good })
            .with(Velocity(Vec3::new(0.0, 0.0, 0.0)));
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
    let delta =  time.delta_seconds();
    for mut velocity in velocities.iter_mut() {
        velocity.0 += Vec3::new(0.0, -1.0, 0.0) * 800.0 * delta;
    }
}
