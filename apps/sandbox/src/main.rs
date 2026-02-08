pub mod voxel;

use bevy::{
    app::TaskPoolThreadAssignmentPolicy,
    prelude::*,
    render::{
        RenderPlugin,
        settings::{Backends, RenderCreation, WgpuFeatures, WgpuSettings},
    },
};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};

use voxel::voxel::create_cube_mesh;

fn draw_grid(mut gizmos: Gizmos) {
    let cell_count = UVec2::new(20, 20);
    let spacing = Vec2::splat(1.0);

    // XZ Plane (Floor)
    gizmos.grid(
        Quat::IDENTITY,
        cell_count,
        spacing,
        Color::srgb(0.0, 1.0, 0.0),
    );

    // YZ Plane (Side Wall)
    gizmos.grid(
        Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
        cell_count,
        spacing,
        Color::srgb(1.0, 0.0, 0.0),
    );

    // XY Plane (Back Wall)
    gizmos.grid(
        Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
        cell_count,
        spacing,
        Color::srgb(0.0, 0.0, 1.0),
    );
}

fn tweak_camera(mut query: Query<&mut Projection, With<Camera>>) {
    for mut projection in &mut query {
        if let Projection::Perspective(ref mut p) = *projection {
            p.near = 0.1;
            p.far = 0.2;
        }
    }
}

fn ui_example_system(mut contexts: EguiContexts) -> Result {
    egui::Window::new("Hello").show(contexts.ctx_mut()?, |ui| {
        ui.label("world");
    });
    Ok(())
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::PRIMARY | Backends::GL),
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    }),
                    ..default()
                })
                .set(TaskPoolPlugin {
                    task_pool_options: TaskPoolOptions {
                        async_compute: TaskPoolThreadAssignmentPolicy {
                            min_threads: 1,
                            max_threads: 8,
                            percent: 0.75,
                            on_thread_spawn: None,
                            on_thread_destroy: None,
                        },
                        ..default()
                    },
                }),
        ))
        .add_plugins(EguiPlugin::default())
        .add_systems(EguiPrimaryContextPass, ui_example_system)
        .add_systems(Startup, (setup, tweak_camera))
        .add_systems(
            Update, draw_grid)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh_handle: Handle<Mesh> = meshes.add(create_cube_mesh());
    // cube
    commands.spawn((
        Mesh3d(cube_mesh_handle),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(3.0, 0.0, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 5., 10.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
