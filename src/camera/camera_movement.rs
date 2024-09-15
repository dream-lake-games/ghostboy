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

#[derive(Component, Clone, Debug, Reflect)]
pub struct FollowDynamicCamera;

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

fn camera_follow_test(
    mut dynamic_camera: Query<&mut Pos, With<DynamicCamera>>,
    test_player: Query<&IPos, (With<TestPlayer>, Without<DynamicCamera>)>,
) {
    let Ok(mut pos) = dynamic_camera.get_single_mut() else {
        warn!("yikes camerafollow");
        return;
    };
    let Ok(test_player_pos) = test_player.get_single() else {
        return;
    };
    pos.x = test_player_pos.cur.x as f32;
    pos.y = test_player_pos.cur.y as f32;
}

fn follow_dynamic_camera(
    dynamic_camera: Query<&Pos, With<DynamicCamera>>,
    mut followers: Query<&mut Transform, (With<FollowDynamicCamera>, Without<DynamicCamera>)>,
) {
    let Ok(leader) = dynamic_camera.get_single() else {
        warn!("yikes followdynamic");
        return;
    };
    for mut tran in &mut followers {
        tran.translation.x = leader.x;
        tran.translation.y = leader.y;
    }
}

pub(super) fn register_camera_movement(app: &mut App) {
    reg_types!(app, DynamicCamera);

    app.add_systems(Startup, spawn_dynamic_camera.after(RootInit));

    app.add_systems(
        PostUpdate,
        (camera_follow_test, follow_dynamic_camera)
            .chain()
            .in_set(CameraSet)
            .after(PhysicsSet),
    );
}
