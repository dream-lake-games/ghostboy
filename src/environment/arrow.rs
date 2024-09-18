use crate::prelude::*;

#[derive(Component, Clone, Debug, Reflect)]
pub struct Arrow;

#[derive(Clone, Debug, Reflect)]
pub struct ArrowDeleted;
impl Component for ArrowDeleted {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let mut anim = world.get_mut::<AnimMan<ArrowAnim>>(eid).unwrap();
            let new_state = match anim.get_state() {
                ArrowAnim::North => ArrowAnim::NorthFade,
                ArrowAnim::East => ArrowAnim::EastFade,
                ArrowAnim::NorthEast => ArrowAnim::NorthEastFade,
                _ => anim.get_state(),
            };
            anim.set_state(new_state);
        });
    }
}

#[derive(Bundle)]
pub struct ArrowBundle {
    name: Name,
    arrow: Arrow,
    anim: AnimMan<ArrowAnim>,
    pos: Pos,
    spatial: SpatialBundle,
    dyno: Dyno,
    trigger_tx: TriggerTx,
    static_rx: StaticRx,
}
impl ArrowBundle {
    pub fn new(pos: Pos, card: CardDir) -> Self {
        let (key, flipx, flipy) = match card {
            CardDir::N => (ArrowAnim::North, false, false),
            CardDir::NE => (ArrowAnim::NorthEast, false, false),
            CardDir::E => (ArrowAnim::East, false, false),
            CardDir::SE => (ArrowAnim::NorthEast, false, true),
            CardDir::S => (ArrowAnim::North, false, true),
            CardDir::SW => (ArrowAnim::NorthEast, true, true),
            CardDir::W => (ArrowAnim::East, true, false),
            CardDir::NW => (ArrowAnim::NorthEast, true, false),
        };
        const ARROW_SPEED: f32 = 60.0;
        Self {
            name: Name::new("arrow"),
            arrow: Arrow,
            anim: AnimMan::new()
                .with_state(key)
                .with_flip_x(flipx)
                .with_flip_y(flipy),
            pos,
            spatial: pos.to_spatial(ZIX_SKELLY - 0.1),
            dyno: Dyno {
                vel: card.as_vec2().normalize_or_zero() * ARROW_SPEED,
            },
            trigger_tx: TriggerTx::single(TriggerTxKind::Arrow, Hbox::new().with_size(7, 7)),
            static_rx: StaticRx::single(StaticRxKind::Default, Hbox::new().with_size(5, 5)),
        }
    }
}

pub(super) fn delete_arrows(
    mut commands: Commands,
    arrows: Query<(Entity, &Pos, &StaticRxCtrl), (With<Arrow>, Without<ArrowDeleted>)>,
    current_level: Res<CurrentLevelHelpers>,
) {
    for (eid, pos, ctrl) in &arrows {
        if (current_level.bounds.is_some()
            && !current_level.bounds.unwrap().contains(pos.as_vec2()))
            || !ctrl.coll_keys.is_empty()
        {
            commands.entity(eid).insert(ArrowDeleted);
        }
    }
}

pub(super) fn register_arrows(app: &mut App) {
    app.add_systems(Update, delete_arrows.in_set(EnemySet).after(PhysicsSet));
}
