use bevy::prelude::*;

struct Materials {
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dBundle::default())
        .insert_resource(Materials {
        });
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "C0DE".into(),
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.15, 0.58, 0.92).into()))
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .run()
}
