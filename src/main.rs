#[cfg(feature = "editor")]
use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::{
    asset::AssetServerSettings,
    prelude::*,
    render::settings::{WgpuFeatures, WgpuSettings},
    window::PresentMode,
};
#[cfg(feature = "editor")]
use bevy_editor_pls::{
    controls::{self, EditorControls},
    EditorPlugin,
};
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        width: 1280.0,
        height: 720.0,
        title: String::from("Bevy Jam 2.0 - Combine"),
        present_mode: PresentMode::Fifo,
        resizable: false,
        decorations: false,
        ..default()
    })
    .insert_resource(AssetServerSettings {
        watch_for_changes: true,
        ..default()
    })
    .insert_resource(WgpuSettings {
        features: WgpuFeatures::POLYGON_MODE_LINE,
        ..default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default());

    #[cfg(feature = "editor")]
    app.add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(EntityCountDiagnosticsPlugin)
        .add_plugin(EditorPlugin)
        .insert_resource(editor_controls())
        .add_startup_system(set_cam3d_controls);

    app.add_plugin(LookTransformPlugin)
        .add_plugin(FpsCameraPlugin::default());

    app.add_startup_system(setup).run();
}

const EYE: Vec3 = Vec3::from_array([-3.0, 3.0, 10.0]);
const TARGET: Vec3 = Vec3::ZERO;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_translation(EYE).looking_at(TARGET, Vec3::Y),
            ..default()
        })
        .insert_bundle(FpsCameraBundle::new(
            FpsCameraController::default(),
            EYE,
            TARGET,
        ));

    commands
        .spawn_bundle((Collider::cuboid(100.0, 0.1, 100.0),))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: -50.0,
                max_x: 50.0,
                min_y: -0.05,
                max_y: 0.05,
                min_z: -50.0,
                max_z: 50.0,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -2.0, 0.0),
            ..default()
        });

    commands
        .spawn_bundle((
            RigidBody::Dynamic,
            Collider::ball(0.5),
            Restitution::coefficient(0.7),
        ))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.5,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::INDIGO,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        });
}

#[cfg(feature = "editor")]
fn editor_controls() -> EditorControls {
    let mut editor_controls = EditorControls::default_bindings();
    editor_controls.unbind(controls::Action::PlayPauseEditor);

    editor_controls.insert(
        controls::Action::PlayPauseEditor,
        controls::Binding {
            input: controls::UserInput::Single(controls::Button::Keyboard(KeyCode::Escape)),
            conditions: vec![controls::BindingCondition::ListeningForText(false)],
        },
    );

    editor_controls
}

#[cfg(feature = "editor")]
fn set_cam3d_controls(
    mut query: Query<
        &mut bevy_editor_pls::default_windows::cameras::camera_3d_free::FlycamControls,
    >,
) {
    let mut controls = query.single_mut();
    controls.key_up = KeyCode::E;
    controls.key_down = KeyCode::Q;
}
