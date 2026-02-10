#[derive(Component)]
struct FreeCam {
    sensitivity: f32,
    speed: f32,
}

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    pbr::wireframe::WireframePlugin,
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};

use engine::debug::*;

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
            p.near = 1.0;
            p.far = 0.2;
        }
    }
}

#[derive(Component)]
struct PanOrbitCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

fn pan_orbit_camera(
    mut ev_motion: MessageReader<MouseMotion>,
    mut ev_scroll: MessageReader<MouseWheel>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &mut PanOrbitCamera)>,
) {
    let mut rotation_move = Vec2::ZERO;
    let mut pan = Vec2::ZERO;
    let mut scroll = 0.0;

    for ev in ev_motion.read() {
        if mouse_input.pressed(MouseButton::Right) {
            rotation_move += ev.delta;
        } else if mouse_input.pressed(MouseButton::Middle) {
            pan += ev.delta;
        }
    }

    for ev in ev_scroll.read() {
        scroll += ev.y;
    }

    for (mut transform, mut orbit) in &mut query {
        // --- Zoom ---
        if scroll.abs() > 0.0 {
            orbit.radius -= scroll * orbit.radius * 0.2;
            orbit.radius = orbit.radius.clamp(2.0, 500.0);
        }

        // --- Orbit ---
        if rotation_move.length_squared() > 0.0 {
            let delta_x = rotation_move.x * 0.005;
            let delta_y = rotation_move.y * 0.005;

            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);

            transform.rotation = yaw * transform.rotation;
            transform.rotation = transform.rotation * pitch;
        }

        // --- Pan ---
        if pan.length_squared() > 0.0 {
            let right = transform.rotation * Vec3::X * -pan.x * 0.01;
            let up = transform.rotation * Vec3::Y * pan.y * 0.01;
            orbit.focus += right + up;
        }

        // --- Update Transform ---
        let rot_matrix = Mat3::from_quat(transform.rotation);
        transform.translation =
            orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, orbit.radius));
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
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::PRIMARY | Backends::GL),
                features: WgpuFeatures::POLYGON_MODE_LINE,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WireframePlugin::default())
        // Note: You can add Egui back here if needed
        .add_systems(Startup, (setup, spawn_test_chunk, spawn_test_chunk_greedy))
        .add_systems(Update, (draw_grid, tweak_camera, pan_orbit_camera))
        .run();
}

fn setup(mut commands: Commands) {
    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 20.0, 4.0),
    ));

    // Camera
    let translation = Vec3::new(-10.0, 20.0, 40.0);
    let focus = Vec3::new(16., 0., 16.);

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(translation).looking_at(focus, Vec3::Y),
        // ADD THIS: The system won't run without this component!
        PanOrbitCamera {
            focus,
            radius: translation.distance(focus),
            upside_down: false,
        },
        FreeCam {
            sensitivity: 0.003,
            speed: 20.0,
        },
    ));
}
