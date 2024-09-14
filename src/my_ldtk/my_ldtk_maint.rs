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

pub(super) fn register_ldtk_maint(app: &mut App) {
    app.add_systems(PreUpdate, post_ldtk_blessing);
}
