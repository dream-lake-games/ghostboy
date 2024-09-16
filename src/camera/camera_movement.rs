use crate::prelude::*;

#[derive(Resource, Clone, Debug, Reflect)]
struct CameraMovementConsts {
    /// What is the max pixels that the user is allowed to be in front of the camera horizontally?
    hor_forward_leash: u32,
    /// What is the max pixels that the user is allowed to be behind the camera horizontally?
    hor_backward_leash: u32,
}
impl Default for CameraMovementConsts {
    fn default() -> Self {
        Self {
            hor_forward_leash: 0,
            hor_backward_leash: SCREEN_WIDTH / 4,
        }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
struct DynamicCamera;

/// This is for the layer cameras that need to "follow" the dynamic camera. Don't get confused
#[derive(Component, Clone, Debug, Reflect)]
pub struct FollowDynamicCamera;

#[derive(Resource, Clone, Copy, Debug, Reflect, Default)]
pub enum DynamicCameraMode {
    /// Follow an entity
    Follow(Entity),
    /// Catch-all for now, don't overthink the API until you need to
    #[default]
    Hanging,
}

#[derive(Bundle)]
struct DynamicCameraBundle {
    name: Name,
    pos: Pos,
    marker: DynamicCamera,
}

fn spawn_dynamic_camera(mut commands: Commands, root: Res<LayerRoot>) {
    commands
        .spawn(DynamicCameraBundle {
            name: Name::new("dynamic_camera"),
            pos: Pos::new(0.0, 0.0),
            marker: DynamicCamera,
        })
        .set_parent(root.eid());
}

fn move_camera(
    mut dynamic_camera: Query<&mut Pos, With<DynamicCamera>>,
    ipos_q: Query<&IPos>,
    mut camera_mode: ResMut<DynamicCameraMode>,
    current_level_helpers: Res<CurrentLevelHelpers>,
) {
    let Ok(mut pos) = dynamic_camera.get_single_mut() else {
        warn!("yikes camerafollow");
        return;
    };

    // First handle mode specific movement
    match *camera_mode {
        DynamicCameraMode::Follow(eid) => {
            match ipos_q.get(eid) {
                Ok(ipos) => {
                    pos.x = ipos.cur.x as f32;
                    pos.y = ipos.cur.y as f32;
                }
                Err(e) => {
                    warn!("Camera following non-existent entity. Going to hang, {e:?}");
                    *camera_mode = DynamicCameraMode::Hanging;
                }
            };
        }
        DynamicCameraMode::Hanging => (),
    }

    // Then snap the camera inside level bounds (common to all camera modes)
    if let Some(bounds) = current_level_helpers.bounds {
        let cam_min_x = pos.x - SCREEN_WIDTH_f32 / 2.0;
        let cam_max_x = pos.x + SCREEN_WIDTH_f32 / 2.0;
        if cam_min_x < bounds.min.x {
            pos.x += bounds.min.x - cam_min_x;
        } else if bounds.max.x < cam_max_x {
            pos.x -= cam_max_x - bounds.max.x;
        }

        let cam_min_y = pos.y - SCREEN_HEIGHT_f32 / 2.0;
        let cam_max_y = pos.y + SCREEN_HEIGHT_f32 / 2.0;
        if cam_min_y < bounds.min.y {
            pos.y += bounds.min.y - cam_min_y;
        } else if bounds.max.y < cam_max_y {
            pos.y -= cam_max_y - bounds.max.y;
        }
    }
}

fn follow_dynamic_camera(
    dynamic_camera: Query<&Pos, With<DynamicCamera>>,
    mut followers: Query<&mut Transform, (With<FollowDynamicCamera>, Without<DynamicCamera>)>,
    camera_shake: Res<CameraShakeOffset>,
) {
    let Ok(leader) = dynamic_camera.get_single() else {
        warn!("yikes followdynamic");
        return;
    };
    for mut tran in &mut followers {
        tran.translation.x = leader.x + camera_shake.offset.x as f32;
        tran.translation.y = leader.y + camera_shake.offset.y as f32;
    }
}

pub(super) fn register_camera_movement(app: &mut App) {
    reg_types!(app, DynamicCamera);

    app.insert_resource(DynamicCameraMode::Hanging);
    app.add_systems(Startup, spawn_dynamic_camera.after(RootInit));

    app.add_systems(
        PostUpdate,
        (move_camera, follow_dynamic_camera)
            .chain()
            .in_set(CameraSet)
            .after(PhysicsSet),
    );
}
