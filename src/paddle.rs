use bevy::{prelude::*, render::view::window, sprite::MaterialMesh2dBundle};

use crate::{position::*, shape::Shape};

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
}

impl PaddleBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            paddle: Paddle,
            position: Position(Vec2::new(x, y)),
            shape: Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
        }
    }
}

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_paddles)
            .add_systems(Update, project_positions);
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
            // player
            MaterialMesh2dBundle {
                material: materials.add(ColorMaterial::from(Color::rgb(0., 1., 0.))),
                mesh: mesh_handle.clone().into(),
                ..default()
            },
            PaddleBundle::new(right_paddle_x, 0.),
        ));

        commands.spawn((
            // ai
            MaterialMesh2dBundle {
                material: materials.add(ColorMaterial::from(Color::rgb(0., 0., 1.))),
                mesh: mesh_handle.into(),
                ..default()
            },
            PaddleBundle::new(left_paddle_x, 0.),
        ));
    }
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in positionables.iter_mut() {
        transform.translation = position.0.extend(0.);
    }
}
