use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_grav)
        .add_startup_system(setup_physics)
        .add_system(movment)
        .run();
}

#[derive(Debug, Component)]
struct Ground {
    width: f32,
    height: f32,
    thickness: f32,
}

impl Ground {
    fn new(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        width: f32,
        height: f32,
        thickness: f32,
        tranform: (f32, f32, f32),
    ) {
        commands
            .spawn(Collider::cuboid(width, height, thickness))
            .insert(TransformBundle::from(Transform::from_xyz(
                tranform.0, tranform.1, tranform.2,
            )));

        commands.spawn(PbrBundle {
            mesh: meshes.add(shape::Box::new(width, height, thickness).into()),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(tranform.0, tranform.1, tranform.2),
            ..default()
        });
    }
}

#[derive(Debug, Component)]
struct Wall {
    width: f32,
    height: f32,
    thickness: f32,
}

impl Wall {
    fn new(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        width: f32,
        height: f32,
        thickness: f32,
        tranform: (f32, f32, f32),
    ) {
        commands
            .spawn(Collider::cuboid(width, height, thickness))
            .insert(TransformBundle::from(Transform::from_xyz(
                tranform.0, tranform.1, tranform.2,
            )));

        commands.spawn(PbrBundle {
            mesh: meshes.add(shape::Box::new(width, height, thickness).into()),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(tranform.0, tranform.1, tranform.2),
            ..default()
        });
    }
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn setup_grav(mut grav: Query<&mut GravityScale>) {
    for mut grav in grav.iter_mut() {
        grav.0 = 0.5
    }
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground = Ground::new(
        commands,
        meshes,
        materials,
        100.0,
        0.1,
        100.0,
        (0.0, -0.1, 0.0),
    );
}

fn movment(mut charater: Query<&mut KinematicCharacterController>) {
    for mut controller in charater.iter_mut() {}
}
