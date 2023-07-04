use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use std::f32::consts::PI;

const BALL_RADIUS: f32 = 30.0;
const BALL_SIZE: Vec3 = Vec3::new(BALL_RADIUS * 2.0, BALL_RADIUS * 2.0, 1.0);

static BALL_POSITIONS: [Vec3; 16] = [
    Vec3::new(10.0 * BALL_RADIUS, 0.0, 0.0), //Cue Ball
    Vec3::new(0.0,                        -2.0 * BALL_RADIUS, 0.0), //Ball 1
    Vec3::new(1.732 * -2.0 * BALL_RADIUS, -2.0 * BALL_RADIUS, 0.0), //Ball 2
    Vec3::new(1.732 * -1.0 * BALL_RADIUS, -1.0 * BALL_RADIUS, 0.0), //Ball 3
    Vec3::new(1.732 * -2.0 * BALL_RADIUS,  2.0 * BALL_RADIUS, 0.0), //Ball 4
    Vec3::new(1.732 * -2.0 * BALL_RADIUS,  4.0 * BALL_RADIUS, 0.0), //Ball 5
    Vec3::new(1.732 * -1.0 * BALL_RADIUS,  3.0 * BALL_RADIUS, 0.0), //Ball 6
    Vec3::new(1.732 *  1.0 * BALL_RADIUS,  1.0 * BALL_RADIUS, 0.0), //Ball 7
    Vec3::new(0.0                       ,  0.0              , 0.0), //Ball 8
    Vec3::new(1.732 *  2.0 * BALL_RADIUS,  0.0              , 0.0), //Ball 9
    Vec3::new(1.732 * -1.0 * BALL_RADIUS,  1.0 * BALL_RADIUS, 0.0), //Ball 10
    Vec3::new(1.732 * -2.0 * BALL_RADIUS, -4.0 * BALL_RADIUS, 0.0), //Ball 11
    Vec3::new(1.732 *  1.0 * BALL_RADIUS, -1.0 * BALL_RADIUS, 0.0), //Ball 12
    Vec3::new(1.732 * -2.0 * BALL_RADIUS,  0.0              , 0.0), //Ball 13
    Vec3::new(1.732 * -1.0 * BALL_RADIUS, -3.0 * BALL_RADIUS, 0.0), //Ball 14
    Vec3::new(0.0                       ,  2.0 * BALL_RADIUS, 0.0)  //Ball 15
];

const BALL_COLORS: [Color; 16] = [
    Color::rgb(1.0, 1.0, 0.9), //Cue Ball
    Color::rgb(0.8, 0.8, 0.0), //Ball 1
    Color::rgb(0.0, 0.0, 0.8), //Ball 2
    Color::rgb(0.8, 0.1, 0.0), //Ball 3
    Color::rgb(0.2, 0.0, 0.3), //Ball 4
    Color::rgb(0.8, 0.4, 0.2), //Ball 5
    Color::rgb(0.0, 0.6, 0.25), //Ball 6
    Color::rgb(0.5, 0.0, 0.15), //Ball 7
    Color::rgb(0.0, 0.0, 0.0), //Ball 8
    Color::rgb(0.8, 0.8, 0.0), //Ball 9
    Color::rgb(0.0, 0.0, 0.8), //Ball 10
    Color::rgb(0.8, 0.1, 0.0), //Ball 11
    Color::rgb(0.2, 0.0, 0.3), //Ball 12
    Color::rgb(0.8, 0.4, 0.2), //Ball 13
    Color::rgb(0.0, 0.6, 0.25), //Ball 14
    Color::rgb(0.5, 0.0, 0.15)  //Ball 15
];

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.3, 0.15);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugin(BilliardsPlugin)
        .add_startup_system(render_setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn render_setup(mut commands: Commands){
    commands.spawn(Camera3dBundle::default());
}

pub struct BilliardsPlugin;

impl Plugin for BilliardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_balls)
            .add_system(ball_physics)
            .add_system(control);

    }
}

//Cue ball is 0
#[derive(Component)]
struct BilliardBall { is_solid: bool, number: u8 }

#[derive(Component)]
struct RigidBody { velocity: Vec2, rotational_velocity: f32 }

fn add_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    println!("adding balls");

    let texture_handle = asset_server.load("striped_ball.png");

    let mut i: usize = 0;

    while i < 16 {
        if i <= 8 {
            commands.spawn(
                (
                BilliardBall { is_solid: true, number: i as u8 },
                PbrBundle {
                    transform: Transform {
                        translation: BALL_POSITIONS[i],
                        scale: BALL_SIZE,
                        ..default()
                    },
                    mesh: meshes.add(shape::UVSphere::default().into()).into(),
                    material: materials.add(
                        StandardMaterial {
                            base_color: BALL_COLORS[i],
                            unlit: true,
                            ..default()
                        }
                    ),
                    ..default() 
                },
                RigidBody {
                    velocity: Vec2::ZERO,
                    rotational_velocity: 0.0
                }
                 )
            );
        } else {
            commands.spawn (
                (
                BilliardBall { is_solid: true, number: i as u8 },
                PbrBundle {
                    transform: Transform {
                        translation: BALL_POSITIONS[i],
                        scale: BALL_SIZE,
                        rotation: Quat::from_rotation_x(PI/2.0),
                        ..default()
                    },
                    mesh: meshes.add(shape::UVSphere::default().into()).into(),
                    material: materials.add(
                        StandardMaterial {
                            base_color: BALL_COLORS[i],
                            base_color_texture: Some(texture_handle.clone()),
                            unlit: true,
                            ..default()
                        }
                    ),
                    ..default()
                },
                RigidBody {
                    velocity: Vec2::ZERO,
                    rotational_velocity: 0.0
                }
                )
            );
        }
        i += 1;
    }   
}

fn ball_physics(time: Res<Time>, query: Query<(&mut Transform, &mut RigidBody)>){
    println!("doing physics");
}

fn control() {
    println!("doing control stuff");
}
