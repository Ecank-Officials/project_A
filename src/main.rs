use bevy::prelude::*;
#[derive(Component, Debug)]
struct Player {
    name: String,
}
#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (player_spawn, enemy_spawn, obstacles_spawn))
        .add_systems(Update, (player_movement, print_postion))
        .run();
}
fn player_spawn(mut commands: Commands) {
    commands.spawn((
        Player {
            name: "player".to_string(),
        },
        Position { x: 0.0, y: 0.0 },
        Velocity { x: 0.0, y: 0.0 },
    ));
}
fn enemy_spawn(mut commands: Commands) {
    commands.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 0.0, y: 0.0 }));
}
fn obstacles_spawn(mut commands: Commands) {
    commands.spawn(Position { x: 0.0, y: 0.0 });
}
fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Position, &mut Velocity)>,
) {
    for (_player, mut position, mut velocity) in query.iter_mut() {
        let mut direction_x = 0.0;
        let mut direction_y = 0.0;
        let g = 0.9;
        if keyboard_input.just_pressed(KeyCode::KeyA) {
            direction_x = -1.0;
        }
        if keyboard_input.just_pressed(KeyCode::KeyD) {
            direction_x = 1.0;
        }
        if keyboard_input.just_pressed(KeyCode::Space) {
            direction_y = 1.0;
        }
        velocity.x = direction_x;
        velocity.y = direction_y;
        position.x += velocity.x * 1.0;
        position.y += velocity.y * 1000.0;
        if velocity.y == 0.0 && position.y > 0.0 {
            position.y -= g;
        }
    }
}

fn print_postion(query: Query<(Entity, &Player, &Position, &Velocity)>) {
    for (entity, player, position, _velocity) in query.iter() {
        info!(
            "Entity{:?}{:?} is at position {:?},",
            entity, player.name, position
        );
    }
}
