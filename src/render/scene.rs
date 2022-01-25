use bevy::{
    input::{
        mouse::{MouseMotion, MouseWheel},
        Input,
    },
    prelude::*,
};

use super::context;

pub fn spawn_base_env(
    mut ctx_ent: ResMut<context::Ent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<StandardMaterial>>,
) {
    // 相机
    let cam = commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0f32, 0f32, 10f32),
            ..Default::default()
        })
        .id();
    ctx_ent.camera = Some(cam);

    // 灯
    let transform = Transform::from_rotation(Quat::from_euler(
        EulerRot::ZXY,
        0f32.to_radians(),
        -30f32.to_radians(),
        15f32.to_radians(),
    ));
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000f32,
            ..Default::default()
        },
        transform,
        ..Default::default()
    });

    // 坐标指示
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(
            shape::Icosphere {
                radius: 0.05f32,
                subdivisions: 1,
            }
            .into(),
        ),
        ..Default::default()
    });
    let l = 1f32;
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(
            shape::Quad {
                size: Vec2::new(0.05, l),
                ..Default::default()
            }
            .into(),
        ),
        transform: Transform::from_xyz(0f32, 0f32, l * 0.5)
            .with_rotation(Quat::from_rotation_x(-90f32.to_radians())),
        ..Default::default()
    });
}

pub fn cam_control(
    mut cam: Query<(Entity, &mut Transform), With<Camera>>,
    time: Res<Time>,
    ctx_ent: Res<context::Ent>,
    ctx_ctr: Res<context::Ctr>,
    mut motion_e: EventReader<MouseMotion>,
    mut wheel_e: EventReader<MouseWheel>,
    key: Res<Input<KeyCode>>,
) {
    if ctx_ent.camera.is_none() {
        return;
    }
    let act_cam = ctx_ent.camera.unwrap();
    let view_speed = ctx_ctr.view_speed;
    for (id, mut cam) in cam.iter_mut() {
        if id == act_cam {
            for motion in motion_e.iter() {
                let rot_x = -motion.delta.y * view_speed;
                let rot_y = -motion.delta.x * view_speed;
                let rot = Quat::from_axis_angle(cam.local_x(), rot_x)
                    * Quat::from_axis_angle(Vec3::Y, rot_y);
                cam.rotate(rot);
            }
            let xn_btns = [KeyCode::A, KeyCode::Left];
            let xp_btns = [KeyCode::D, KeyCode::Right];
            let zn_btns = [KeyCode::S, KeyCode::Down];
            let zp_btns = [KeyCode::W, KeyCode::Up];
            let yn_btns = [KeyCode::Q];
            let yp_btns = [KeyCode::E];
            let x = if key.any_pressed(xn_btns) {
                -1
            } else if key.any_pressed(xp_btns) {
                1
            } else {
                0
            } as f32;
            let z = if key.any_pressed(zn_btns) {
                -1
            } else if key.any_pressed(zp_btns) {
                1
            } else {
                0
            } as f32;

            let y = if key.any_pressed(yn_btns) {
                -1
            } else if key.any_pressed(yp_btns) {
                1
            } else {
                0
            } as f32;
            let trans = cam.local_x() * x + -cam.local_z() * z + cam.local_y() * y;
            cam.translation += trans * time.delta_seconds() * ctx_ctr.move_speed;
            break;
        }
    }
}
