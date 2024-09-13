//! It's pet peeve of mine to have a disorganized hierarchy in the debugger.
//! It looks bad, and when stuff goes wrong it makes it much harder to actually
//! figure out what's wrong.

use crate::prelude::*;

macro_rules! impl_root_types {
    ($name:ident) => {
        paste::paste! {
            #[derive(Component, Debug, Reflect)]
            pub struct[<$name Component>];

            #[derive(Bundle)]
            pub struct[<$name Bundle>] {
                name: Name,
                marker: [<$name Component>],
                spatial: SpatialBundle,
            }
            impl [<$name Bundle>] {
                fn new(pos: Pos, zix: Zix) -> Self {
                    Self {
                        name: Name::new(stringify!($name)),
                        marker: [<$name Component>],
                        spatial: pos_zix_to_spat(pos, zix),
                    }
                }
            }

            #[derive(Resource, Debug, Reflect)]
            pub struct $name {
                eid: Entity,
            }
            impl $name {
                pub fn eid(&self) -> Entity {
                    self.eid
                }
            }
        }
    };
}

macro_rules! impl_root_init {
    ($($name:ident$(($zix:expr))?),*) => {
        $(
            impl_root_types!($name);
        )*

        paste::paste! {
            fn setup_roots(
                mut commands: Commands,
                $(
                    #[allow(nonstandard_style)]
                    mut [<$name _res>]: ResMut<$name>,
                )*
            ) {
                $(
                    let pos = Pos::default();
                    #[allow(unused_mut, unused_assignments)]
                    let mut zix = Zix::default();
                    $(
                        zix = $zix;
                    )?
                    #[allow(nonstandard_style)]
                    let [<$name _eid>] = commands.spawn([<$name Bundle>]::new(pos, zix)).id();
                    [<$name _res>].eid = [<$name _eid>];
                )*
            }
        }

        #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
        pub struct RootInit;

        pub(super) struct RootPlugin;
        impl Plugin for RootPlugin {
            fn build(&self, app: &mut App) {
                $(
                    app.insert_resource($name {
                        eid: Entity::PLACEHOLDER,
                    });
                )*

                app.add_systems(Startup, setup_roots.in_set(RootInit));
            }
        }
    };
}

impl_root_init!(
    CollisionRoot,
    DebugRoot,
    LayerRoot,
    MenuRoot(ZIX_MENU),
    ParticlesRoot(ZIX_PARTICLES),
    RoomRoot,
    SoundRoot,
    TransitionRoot(ZIX_TRANSITION),
    TutorialRoot
);
