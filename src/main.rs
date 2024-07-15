use bevy::prelude::*;
#[derive(Component)]
struct Player {
    name: String,
}
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_systems(Update, player_movement)
        .run();
}
fn init(mut commands: Commands) {
    commands.spawn((
        Player {
            name: "player".to_string(),
        },
        Position { x: 0.0, y: 0.0 },
        Velocity { x: 0.0, y: 0.0 },
    ));
}
fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Position, &mut Velocity)>,
) {
    for (_player, mut position, mut velocity) in query.iter_mut() {
        let mut direction_x = 0.0;

        if keyboard_input.just_pressed(KeyCode::KeyA) {
            direction_x -= 1.0;
        }
        if keyboard_input.just_pressed(KeyCode::KeyD) {
            direction_x += 1.0;
        }
        velocity.x = direction_x;

        position.x += velocity.x * 1.0;
    }
}
