use bevy::ecs::schedule::ScheduleLabel;

use crate::prelude::*;

/// A schedule that will run every FRAMERATE of IN-GAME time
/// So things like drag will be applied consistently in and out of bullet time
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BulletUpdate;

/// How much in-game time has happened. Basically time but accounts for slowdown.
#[derive(Resource, Debug, Clone, Reflect)]
pub struct BulletTime {
    time_factor: f32,
    main_duration: Duration,
}
impl BulletTime {
    const NORMAL: f32 = 1.0;
    const SLOW: f32 = 0.3;
    const STOPPED: f32 = 0.0;

    pub fn new() -> Self {
        Self {
            time_factor: 1.0,
            main_duration: Duration::default(),
        }
    }

    pub fn delta(&self) -> Duration {
        self.main_duration
    }
    pub fn delta_seconds(&self) -> f32 {
        self.main_duration.as_secs_f32()
    }

    pub fn set_normal(&mut self) {
        self.set_time_factor(Self::NORMAL);
    }
    pub fn set_slow(&mut self) {
        self.set_time_factor(Self::SLOW);
    }
    pub fn set_stopped(&mut self) {
        self.set_time_factor(Self::STOPPED);
    }
    pub fn set_time_factor(&mut self, factor: f32) {
        self.time_factor = factor;
    }
}

fn update_bullet_time(mut bullet_time: ResMut<BulletTime>, time: Res<Time>) {
    bullet_time.main_duration = time.delta().mul_f32(bullet_time.time_factor);
}

fn drive_bullet_time(mut _bullet_time: ResMut<BulletTime>) {}

/// The resource tracking passage of in-game time to drive the BulletUpdate
#[derive(Resource)]
struct InGameTimePassed(f32);

fn shephard_bullet_update(world: &mut World) {
    let in_game_time = world.resource::<BulletTime>().delta_seconds();
    let mut time_passed = world.resource_mut::<InGameTimePassed>();
    time_passed.0 += in_game_time;
    if time_passed.0 >= 1.0 / FRAMERATE {
        time_passed.0 -= 1.0 / FRAMERATE;
        world.run_schedule(BulletUpdate);
    }
}

#[derive(Event)]
pub struct LimitedBulletTime(pub f32);

fn start_limited_bullet_time(
    trigger: Trigger<LimitedBulletTime>,
    mut existing: Query<&mut LimitedActor>,
    mut commands: Commands,
) {
    match existing.get_single_mut() {
        Ok(mut actor) => {
            actor.0 = trigger.event().0;
        }
        Err(_e) => {
            commands.spawn((
                Name::new("limited_bullet_time"),
                LimitedActor(trigger.event().0),
            ));
        }
    }
}

fn update_limited_bullet_time(
    mut commands: Commands,
    mut existing: Query<(Entity, &mut LimitedActor)>,
    time: Res<Time>,
) {
    if let Ok((eid, mut actor)) = existing.get_single_mut() {
        actor.0 -= time.delta_seconds();
        if actor.0 <= 0.0 {
            commands.entity(eid).despawn_recursive();
        }
    }
}

struct LimitedActor(f32);
impl Component for LimitedActor {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, _eid, _| {
            let mut bullet_time = world.resource_mut::<BulletTime>();
            bullet_time.set_slow();
        });
        hooks.on_remove(|mut world, _eid, _| {
            let mut bullet_time = world.resource_mut::<BulletTime>();
            bullet_time.set_normal();
        });
    }
}

pub(super) struct BulletTimePlugin;
impl Plugin for BulletTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BulletTime::new());
        app.add_systems(First, update_bullet_time);
        app.add_systems(Update, drive_bullet_time);
        app.init_schedule(BulletUpdate);
        app.insert_resource(InGameTimePassed(0.0));
        app.add_systems(Update, shephard_bullet_update.in_set(PhysicsSet));
        app.observe(start_limited_bullet_time);
        app.add_systems(Update, update_limited_bullet_time);
    }
}
