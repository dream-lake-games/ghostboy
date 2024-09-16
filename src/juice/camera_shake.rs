use crate::prelude::*;

const SHAKE_EVERY: f32 = 0.05;
const SHAKE_MAG: i32 = 1;

#[derive(Clone, Debug, Reflect, Default)]
struct ShakeData {
    time_left: f32,
    time_since_last_move: f32,
}

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct CameraShakeOffset {
    pub offset: IVec2,
}

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct CameraShake {
    data: Option<ShakeData>,
}
impl CameraShake {
    pub fn start_shake(&mut self, time: f32) {
        self.data = Some(ShakeData {
            time_left: time,
            time_since_last_move: 0.0,
        });
    }
}

fn update_camera_shake(
    mut camera_shake: ResMut<CameraShake>,
    bullet_time: Res<BulletTime>,
    mut camera_shake_offset: ResMut<CameraShakeOffset>,
) {
    let (stop, move_again) = match camera_shake.data.as_mut() {
        Some(shake) => {
            shake.time_left -= bullet_time.delta_seconds();
            if shake.time_left <= 0.0 {
                (true, false)
            } else {
                shake.time_since_last_move += bullet_time.delta_seconds();
                let move_again = shake.time_since_last_move > SHAKE_EVERY;
                (false, move_again)
            }
        }
        None => (false, false),
    };
    if stop {
        camera_shake.data = None;
        camera_shake_offset.offset = IVec2::ZERO;
    } else {
        if move_again {
            camera_shake_offset.offset = IVec2 {
                x: thread_rng().gen_range(-SHAKE_MAG..=SHAKE_MAG),
                y: thread_rng().gen_range(-SHAKE_MAG..=SHAKE_MAG),
            };
        }
    }
}

pub(super) fn register_camera_shake(app: &mut App) {
    app.insert_resource(CameraShakeOffset::default());
    app.insert_resource(CameraShake::default());
    app.add_systems(Update, update_camera_shake);
}
