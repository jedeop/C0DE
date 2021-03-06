use bevy::prelude::*;
pub struct Velocity(pub Vec3);

pub fn movement(time: Res<Time>, mut q: Query<(&mut Transform, &Velocity)>) {
    let delta = time.delta_seconds();

    for (mut transform, velocity) in q.iter_mut() {
        transform.translation += velocity.0 * delta;
    }
}
