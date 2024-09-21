use crate::prelude::*;

/// Exists because ldtk kinda does weird stuff
#[derive(Component, Default)]
pub struct MyLdtkWait {
    extra_offset: Option<Vec2>,
    dyno: Option<Dyno>,
    parent_render_layers: Option<RenderLayers>,
}
macro_rules! with_wait_option {
    ($field:ident, $type:ty) => {
        paste::paste! {
            pub fn [<with_ $field>](mut self, val: $type) -> Self {
                self.$field = Some(val);
                self
            }
        }
    };
}
impl MyLdtkWait {
    pub fn extra_offset(offset: Vec2) -> Self {
        Self {
            extra_offset: Some(offset),
            ..default()
        }
    }
    pub fn dyno(dyno: Dyno) -> Self {
        Self {
            dyno: Some(dyno),
            ..default()
        }
    }
    pub fn parent_render_layers(render_layers: RenderLayers) -> Self {
        Self {
            parent_render_layers: Some(render_layers),
            ..default()
        }
    }

    with_wait_option!(extra_offset, Vec2);
    with_wait_option!(dyno, Dyno);
    with_wait_option!(parent_render_layers, RenderLayers);
}

pub trait MyLdtkReplacable: Bundle + Queryable {
    fn from_pos(pos: Pos) -> Self;
}

#[derive(Component, LdtkIntCell)]
pub struct MyLdtkReplace<B: MyLdtkReplacable> {
    _pd: Option<B>,
}
impl<B: MyLdtkReplacable> Default for MyLdtkReplace<B> {
    fn default() -> Self {
        Self { _pd: None }
    }
}

fn post_ldtk_blessing(
    mut commands: Commands,
    ents: Query<(Entity, &GlobalTransform, &MyLdtkWait, Option<&Parent>)>,
) {
    for (eid, gtran, wait, parent) in &ents {
        commands.entity(eid).remove::<MyLdtkWait>();
        let mut pos = Pos::new(gtran.translation().x, gtran.translation().y);
        if let Some(extra_offset) = wait.extra_offset {
            pos.x += extra_offset.x;
            pos.y += extra_offset.y;
        }
        commands.entity(eid).insert(pos);
        if let Some(dyno) = wait.dyno.clone() {
            commands.entity(eid).insert(dyno);
        }
        if let Some(rl) = wait.parent_render_layers.clone() {
            if let Some(rent) = parent {
                commands.entity(rent.get()).insert(rl);
            }
        }
    }
}

fn post_ldtk_replacement<B: MyLdtkReplacable>(
    mut commands: Commands,
    ents: Query<(Entity, &GlobalTransform, &MyLdtkReplace<B>)>,
    root: Res<LevelRoot>,
) {
    for (eid, gtran, _replace) in &ents {
        let pos = Pos::new(gtran.translation().x, gtran.translation().y);
        let id = commands
            .spawn(B::from_pos(pos))
            .set_parent(root.eid())
            .insert(pos.to_spatial(1.2))
            .id();
        commands.entity(eid).remove::<MyLdtkReplace<B>>();
        commands
            .entity(eid)
            .insert(LdtkDependents { ents: vec![id] });
    }
}

pub fn register_replaceable<B: MyLdtkReplacable>(app: &mut App) {
    app.add_systems(PreUpdate, post_ldtk_replacement::<B>);
}

/// For spawners who's children should die when they die
#[derive(Clone, Default, Reflect, Debug)]
pub struct LdtkDependents {
    ents: Vec<Entity>,
}
impl LdtkDependents {
    pub fn push(&mut self, ent: Entity) {
        self.ents.push(ent);
    }
}
impl Component for LdtkDependents {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_remove(|mut world, eid, _| {
            let deps = world
                .get::<LdtkDependents>(eid)
                .map(|thing| thing.ents.clone())
                .unwrap_or(vec![]);
            for dep in deps {
                world.commands().entity(dep).despawn_recursive();
            }
        });
    }
}

pub(super) fn register_ldtk_maint(app: &mut App) {
    app.add_systems(PreUpdate, post_ldtk_blessing);
}
