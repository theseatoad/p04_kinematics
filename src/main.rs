use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PresentMode};
use ik::SegmentBundle;
pub mod ik;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "p04_ik".into(),
                resolution: (800., 800.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // spawn camera
    commands.spawn(Camera2dBundle::default());
    // spawn tenticle
    commands.spawn(SegmentBundle::new(
        0.0,
        0.0,
        10.0,
        45.0_f32.to_radians(),
        materials.add(ColorMaterial::from(Color::PURPLE)).into(),
        bevy::sprite::Mesh2dHandle(
            meshes.add(Mesh::from(shape::Quad::new(Vec2 { x: 10.0, y: 1.0 }))),
        ),
    ));
}
