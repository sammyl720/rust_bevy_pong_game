use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    bot::Bot, player::Player, position::*, shape::Shape, velocity::Velocity, GUTTER_HEIGHT,
};

const PADDLE_SPEED: f32 = 1.;
const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 50.;

#[derive(Component)]
struct Paddle;

#[derive(Bundle)]
struct PaddleBundle {
    paddle: Paddle,
    position: Position,
    shape: Shape,
    velocity: Velocity,
}

impl PaddleBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            paddle: Paddle,
            position: Position(Vec2::new(x, y)),
            shape: Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            velocity: Velocity(Vec2::new(0., 0.)),
        }
    }
}

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_paddles).add_systems(
            Update,
            (handle_player_input, move_paddles, project_positions).chain(),
        );
    }
}

fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    println!("Spawning paddles ...");

    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();
        let padding = 50.;
        let right_paddle_x = window_width / 2. - padding;
        let left_paddle_x = -window_width / 2. + padding;

        //update?
        let mesh = Mesh::from(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));

        let mesh_handle = meshes.add(mesh);

        commands.spawn((
            Player,
            MaterialMesh2dBundle {
                material: materials.add(ColorMaterial::from(Color::rgb(0., 1., 0.))),
                mesh: mesh_handle.clone().into(),
                ..default()
            },
            PaddleBundle::new(right_paddle_x, 0.),
        ));

        commands.spawn((
            Bot,
            MaterialMesh2dBundle {
                material: materials.add(ColorMaterial::from(Color::rgb(0., 0., 1.))),
                mesh: mesh_handle.into(),
                ..default()
            },
            PaddleBundle::new(left_paddle_x, 0.),
        ));
    }
}

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = paddle.get_single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            velocity.0.y = 3.;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            velocity.0.y = -3.;
        } else {
            velocity.0.y = 0.;
        }
    } else {
    }
}

fn move_paddles(
    mut paddle: Query<(&mut Position, &Velocity), With<Paddle>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let window_height = window.resolution.height();
        let max_y = window_height / 2. - GUTTER_HEIGHT - PADDLE_HEIGHT / 2.;

        for (mut position, velocity) in &mut paddle {
            let new_position = position.0 + velocity.0 * PADDLE_SPEED;
            if new_position.y.abs() < max_y {
                position.0 = new_position;
            }
        }
    }
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position), With<Paddle>>) {
    for (mut transform, position) in positionables.iter_mut() {
        // println!("Paddle position: {:?}", position.0);
        transform.translation = position.0.extend(0.);
    }
}
