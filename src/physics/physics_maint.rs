use crate::prelude::*;

pub(super) trait PhysicsComp: Queryable + Component {
    type Kind: Queryable;
    type Ctrl: PhysicsCtrl;

    fn from_data(ctrl: Entity, kind: Self::Kind, hbox: Hbox) -> Self;
}
pub(super) trait PhysicsCtrl: Queryable + Component + Default {
    fn add_comp(&mut self, eid: Entity);
}
macro_rules! impl_physics_comp {
    ($kind:ty, $comp:ty, $ctrl:ty) => {
        impl Queryable for $kind {}
        impl Queryable for $comp {}
        impl Queryable for $ctrl {}
        impl PhysicsComp for $comp {
            type Kind = $kind;
            type Ctrl = $ctrl;

            fn from_data(ctrl: Entity, kind: $kind, hbox: Hbox) -> Self {
                Self { kind, ctrl, hbox }
            }
        }
        impl PhysicsCtrl for $ctrl {
            fn add_comp(&mut self, eid: Entity) {
                self.comps.push(eid);
            }
        }
    };
}
pub(super) use impl_physics_comp;

#[derive(Clone, Debug, Reflect)]
pub(super) struct PhysicsComps<T: PhysicsComp> {
    data: Vec<(T::Kind, Hbox)>,
}
impl<T: PhysicsComp> PhysicsComps<T> {
    pub fn new(data: Vec<(T::Kind, Hbox)>) -> Self {
        Self { data }
    }
}
impl<T: PhysicsComp> Component for PhysicsComps<T> {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let data = world
                .get::<Self>(eid)
                .expect("Can't get PhysicsComp after spawning")
                .data
                .clone();
            let mut comp_eids = vec![];
            for (ix, (kind, hbox)) in data.into_iter().enumerate() {
                let comp_eid = world
                    .commands()
                    .spawn((
                        Name::new(format!("PhysicsComp_{kind:?}_{ix}")),
                        T::from_data(eid, kind, hbox),
                    ))
                    .set_parent(eid)
                    .id();
                comp_eids.push(comp_eid);
            }
            let mut ctrl = world
                .get_mut::<T::Ctrl>(eid)
                .expect("PhysicsComp spawned without control");
            for comp_eid in comp_eids {
                ctrl.add_comp(comp_eid);
            }
            // 🫡
            world.commands().entity(eid).remove::<Self>();
        });
    }
}
