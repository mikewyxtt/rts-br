use bevy::prelude::*;

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: env!("CARGO_PKG_NAME").to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, fixed_update)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera,
    ));
}

fn fixed_update(mut query: Query<&mut Transform, With<MainCamera>>, keyboard: Res<ButtonInput<KeyCode>>) {
    let mut transform = query.single_mut().unwrap();
    
    let cam_speed = {
        if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
            0.3
        }
        else {
            0.15
        }
    };

    if keyboard.pressed(KeyCode::KeyW) {
        transform.translation.z -= cam_speed;
    }

    if keyboard.pressed(KeyCode::KeyS) {
        transform.translation.z += cam_speed;
    }

    if keyboard.pressed(KeyCode::KeyA) {
        transform.translation.x -= cam_speed;
    }

    if keyboard.pressed(KeyCode::KeyD) {
        transform.translation.x += cam_speed;

    }
}
