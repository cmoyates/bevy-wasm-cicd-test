use ::bevy::prelude::*;
use bevy::{app::AppExit, sprite::MaterialMesh2dBundle, window::PresentMode};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Movement Demo".to_string(),
                present_mode: PresentMode::AutoNoVsync, // Reduces input lag.
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player_movement)
        .run()
}

#[derive(Component)]
pub struct Player {
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::RegularPolygon::new(20.0, 3).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Player {
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
        },
    ));
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub const PLAYER_SPEED: f32 = 400.0;
pub const PLAYER_STEERING_SCALE: f32 = 0.1;

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
    mut exit: EventWriter<AppExit>,
) {
    if let Ok((mut player_transform, mut player)) = player_query.get_single_mut() {
        let mut direction = Vec2::ZERO;

        // Escape to exit
        if keyboard_input.pressed(KeyCode::Escape) {
            exit.send(AppExit);
        }

        // Arrow keys to move
        if keyboard_input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        let desired_velocity = direction * PLAYER_SPEED;
        let steering = (desired_velocity - player.velocity) * PLAYER_STEERING_SCALE;
        player.acceleration = steering;

        let new_velocity = player.velocity + player.acceleration;
        player.velocity = new_velocity;
        player_transform.translation += player.velocity.extend(0.0) * time.delta_seconds();

        // Rotate the player to face the direction of movement
        player_transform.rotation = Quat::from_rotation_z(
            player.velocity.y.atan2(player.velocity.x) - std::f32::consts::FRAC_PI_2,
        );
    }
}
