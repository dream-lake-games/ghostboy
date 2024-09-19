use crate::prelude::*;

#[derive(Resource, Clone, Debug, Reflect)]
struct StormConsts {
    // How long is the screen (almost) white?
    flash_time: f32,
    // How long is every color -1 shade?
    reverb_time: f32,
}
impl Default for StormConsts {
    fn default() -> Self {
        Self {
            flash_time: 0.1,
            reverb_time: 0.2,
        }
    }
}

fn startup_storm(mut commands: Commands) {
    commands.spawn((
        Name::new("steady_rain"),
        Pos::new(0.0, 0.0).to_spatial(ZIX_RAIN),
        AnimMan::<RainAnim>::new(),
    ));
}

#[derive(Component)]
struct LightningComp {
    time_alive: f32,
}

#[derive(Bundle)]
struct LightningBundle {
    name: Name,
    light: LightningComp,
    spatial: SpatialBundle,
    anim: AnimMan<LightningAnim>,
}
impl LightningBundle {
    fn random() -> Self {
        Self {
            name: Name::new("lightning"),
            light: LightningComp { time_alive: 0.0 },
            spatial: Pos::new(0.0, 0.0).to_spatial(ZIX_RAIN),
            anim: AnimMan::new().with_state(LightningAnim::random()),
        }
    }
}

#[derive(Event)]
pub struct Lightning;

fn start_lightning(
    _trigger: Trigger<Lightning>,
    mut commands: Commands,
    existing: Query<Entity, With<LightningComp>>,
) {
    for eid in &existing {
        commands.entity(eid).despawn_recursive();
    }
    commands.spawn(LightningBundle::random());
}

fn update_lightning(
    mut light_q: Query<(Entity, &mut LightningComp)>,
    mut remaps: ResMut<ShadeRemaps>,
    bullet_time: Res<BulletTime>,
    consts: Res<StormConsts>,
    mut commands: Commands,
) {
    let Ok((eid, mut light)) = light_q.get_single_mut() else {
        return;
    };
    light.time_alive += bullet_time.delta_seconds();
    if light.time_alive < consts.flash_time {
        remaps.set(QColor::Color2, QColor::Color1);
        remaps.set(QColor::Color3, QColor::Color1);
        remaps.set(QColor::Color4, QColor::Color2);
    } else if light.time_alive < consts.flash_time + consts.reverb_time {
        remaps.set(QColor::Color2, QColor::Color1);
        remaps.set(QColor::Color3, QColor::Color2);
        remaps.set(QColor::Color4, QColor::Color3);
    } else {
        *remaps = default();
        commands.entity(eid).despawn_recursive();
    }
}

pub(super) fn register_storm(app: &mut App) {
    app.insert_resource(StormConsts::default());
    debug_resource!(app, StormConsts);

    app.add_systems(OnEnter(MetaStateKind::Level), startup_storm);
    app.add_systems(Update, update_lightning);
    app.observe(start_lightning);
}
