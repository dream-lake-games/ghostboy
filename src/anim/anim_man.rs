use bevy::sprite::Mesh2dHandle;

use crate::prelude::*;

use super::{anim_mat::AnimMat, ManagersSet};

#[derive(Debug, Clone, Reflect)]
pub struct AnimBodyData {
    pub path: String,
    pub size: UVec2,
    pub length: u32,
    pub fps: f32,
    pub offset: IVec2,
    pub zix: f32,
    pub scale: IVec2,
    pub render_layers: RenderLayers,
}
impl AnimBodyData {
    fn with_overrides(mut self, overrides: AnimBodyDataOverrides) -> Self {
        self.fps = overrides.override_fps.unwrap_or(self.fps);
        self.offset = overrides.override_pos.unwrap_or(self.offset);
        self.zix = overrides.override_fzix.unwrap_or(self.zix);
        self.scale = overrides.override_scale.unwrap_or(self.scale);
        self
    }
}

#[derive(Default, Debug, Clone, Reflect)]
pub struct AnimBodyDataOverrides {
    pub override_fps: Option<f32>,
    pub override_pos: Option<IVec2>,
    pub override_fzix: Option<f32>,
    pub override_scale: Option<IVec2>,
}

pub trait AnimBody: Queryable + std::hash::Hash + PartialEq + Eq + Copy {
    fn to_body_data(&self) -> AnimBodyData;
}

#[derive(Debug, Clone, Reflect, PartialEq)]
pub enum AnimNextState<NextType> {
    None,
    Some(NextType),
    Despawn,
}

#[derive(Debug, Clone, Reflect)]
pub struct AnimStateData<NextType, BodyType: AnimBody> {
    pub overwritten_bodies: Vec<(BodyType, AnimBodyDataOverrides)>,
    pub next: AnimNextState<NextType>,
}

pub trait AnimStateMachine: Queryable + Default + PartialEq + Eq + Copy {
    type BodyType: AnimBody;

    fn to_state_data(&self) -> AnimStateData<Self, Self::BodyType>;
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct AnimMan<StateMachine: AnimStateMachine> {
    pub state: StateMachine,
    pub hidden: bool,
    pub flip_x: bool,
    pub flip_y: bool,
}
impl<StateMachine: AnimStateMachine> AnimMan<StateMachine> {
    pub fn new() -> Self {
        Self {
            state: default(),
            hidden: false,
            flip_x: false,
            flip_y: false,
        }
    }

    impl_get_copy!(state, StateMachine);
    impl_with!(state, StateMachine);
    impl_get_copy!(hidden, bool);
    impl_with!(hidden, bool);
    impl_get_copy!(flip_x, bool);
    impl_with!(flip_x, bool);
    impl_get_copy!(flip_y, bool);
    impl_with!(flip_y, bool);
}
// This mutability hack exists so that `Changed` has good meaning.
// In bevy, dereferencing a mutable pointer trigggers change. So we want to have a way to make
// set and what not not actually trigger Change unless the values, well, change.
macro_rules! impl_mutable_animation_manager_field {
    ($field:ident, $type:ty) => {
        paste::paste! {
            fn [<set_ $field>](&mut self, val: $type) {
                if val == self.$field {
                    return;
                }
                self.$field = val;
            }
            fn [<reset_ $field>](&mut self, val: $type) {
                self.$field = val;
            }
        }
    };
}
pub trait MutableAnimationManagerActions<StateMachine: AnimStateMachine> {
    /// Sets the currently value of the animation manager state, doing nothing if the value is the same
    fn set_state(&mut self, state: StateMachine);
    /// Resets the currently value of the animation manager state, triggering change even if the value is the same
    fn reset_state(&mut self, state: StateMachine);
    /// Sets the currently value of the animation manager hidden, doing nothing if the value is the same
    fn set_hidden(&mut self, hidden: bool);
    /// Resets the currently value of the animation manager hidden, triggering change even if the value is the same
    fn reset_hidden(&mut self, hidden: bool);
    /// Sets the currently value of the animation manager flip_x, doing nothing if the value is the same
    fn set_flip_x(&mut self, flip_x: bool);
    /// Resets the currently value of the animation manager flip_x, triggering change even if the value is the same
    fn reset_flip_x(&mut self, flip_x: bool);
    /// Sets the currently value of the animation manager flip_y, doing nothing if the value is the same
    fn set_flip_y(&mut self, flip_y: bool);
    /// Resets the currently value of the animation manager flip_y, triggering change even if the value is the same
    fn reset_flip_y(&mut self, flip_y: bool);
}
impl<'w, StateMachine: AnimStateMachine> MutableAnimationManagerActions<StateMachine>
    for Mut<'w, AnimMan<StateMachine>>
{
    impl_mutable_animation_manager_field!(state, StateMachine);
    impl_mutable_animation_manager_field!(hidden, bool);
    impl_mutable_animation_manager_field!(flip_x, bool);
    impl_mutable_animation_manager_field!(flip_y, bool);
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct AnimBodyProgress<StateMachine: AnimStateMachine> {
    pub ixes: HashMap<StateMachine::BodyType, u32>,
}
impl<StateMachine: AnimStateMachine> AnimBodyProgress<StateMachine> {
    pub fn get_body_ix(&self, body_type: StateMachine::BodyType) -> Option<u32> {
        self.ixes.get(&body_type).map(|thing| *thing)
    }
}

/// For tracking animations that play
#[derive(Component, Debug, Clone, Reflect)]
struct AnimIndex<StateMachine: AnimStateMachine> {
    body_type: StateMachine::BodyType,
    ix: u32,
    length: u32,
    time: f32,
    /// Seconds per frame
    spf: f32,
    /// The state to transition to after this state. Note that this has a None variant inside it.
    next: AnimNextState<StateMachine>,
}

/// Attached to the body of the animation that (when finished) triggers the state transition
#[derive(Component, Debug, Clone, Reflect)]
struct AnimNextBurden<StateMachine: AnimStateMachine> {
    next_state: AnimNextState<StateMachine>,
}

#[derive(Bundle, Clone)]
struct AnimBodyDataBundle<StateMachine: AnimStateMachine> {
    name: Name,
    mesh: Mesh2dHandle,
    material: Handle<AnimMat>,
    spatial: SpatialBundle,
    render_layers: RenderLayers,
    index: AnimIndex<StateMachine>,
}
impl<StateMachine: AnimStateMachine> AnimBodyDataBundle<StateMachine> {
    fn new(
        body_type: StateMachine::BodyType,
        data: AnimBodyData,
        next: AnimNextState<StateMachine>,
        ass: &Res<AssetServer>,
        meshes: &mut ResMut<Assets<Mesh>>,
        mats: &mut ResMut<Assets<AnimMat>>,
    ) -> Self {
        let mesh = Mesh::from(Rectangle::new(data.size.x as f32, data.size.y as f32));
        Self {
            name: Name::new("body_data_bundle"),
            mesh: meshes.add(mesh).into(),
            material: mats.add(AnimMat::new(
                ass.load(data.path),
                data.length,
                false,
                false,
                IVec2::ONE,
            )),
            spatial: SpatialBundle::from_transform(Transform {
                translation: data.offset.as_vec2().extend(data.zix),
                scale: data.scale.extend(1).as_vec3(),
                ..default()
            }),
            render_layers: data.render_layers,
            index: AnimIndex {
                body_type,
                ix: 0,
                length: data.length,
                time: 0.0,
                spf: 1.0 / data.fps,
                next,
            },
        }
    }
}

fn handle_manager_changes<StateMachine: AnimStateMachine>(
    mut commands: Commands,
    managers: Query<
        (Entity, &AnimMan<StateMachine>, Option<&Children>),
        Changed<AnimMan<StateMachine>>,
    >,
    relevant_children: Query<Entity, With<AnimIndex<StateMachine>>>,
    ass: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<AnimMat>>,
) {
    for (eid, manager, ochildren) in &managers {
        if let Some(children) = ochildren {
            for child in children {
                if relevant_children.contains(*child) {
                    commands.entity(*child).despawn_recursive();
                }
            }
        }
        let mut new_progress_map = HashMap::new();
        let state_data = manager.get_state().to_state_data();
        for (ix, (body, overwrite)) in state_data.overwritten_bodies.into_iter().enumerate() {
            new_progress_map.insert(body, 0);
            let data = body.to_body_data().with_overrides(overwrite);
            let next = if ix == 0 {
                state_data.next.clone()
            } else {
                AnimNextState::None
            };
            let body_bund = AnimBodyDataBundle::new(body, data, next, &ass, &mut meshes, &mut mats);
            commands.spawn(body_bund).set_parent(eid);
        }
        commands
            .entity(eid)
            .insert(AnimBodyProgress::<StateMachine> {
                ixes: new_progress_map,
            });
    }
}

fn play_animations<StateMachine: AnimStateMachine>(
    mut commands: Commands,
    mut managers: Query<(
        Entity,
        &mut AnimMan<StateMachine>,
        &mut AnimBodyProgress<StateMachine>,
        &mut Visibility,
    )>,
    mut bodies: Query<(&mut AnimIndex<StateMachine>, &Handle<AnimMat>, &Parent)>,
    mut mats: ResMut<Assets<AnimMat>>,
    bullet_time: Res<BulletTime>,
) {
    for (mut index, hand, parent) in &mut bodies {
        let (manager_eid, mut manager, mut progress, mut visibility) =
            managers.get_mut(parent.get()).unwrap();
        if manager.hidden {
            continue;
        }
        index.time += bullet_time.delta_seconds();
        if index.time < index.spf {
            // No update is happening to this body, can just continue
            continue;
        }
        index.time = 0.0;
        if index.ix + 1 < index.length {
            // Progressing to the next frame of the animation
            index.ix += 1;
            let mat = mats.get_mut(hand.id()).unwrap();
            mat.set_ix(index.ix);
        } else {
            match &index.next {
                AnimNextState::None => {
                    // Looping the animation
                    if index.length <= 1 {
                        // Degen animations don't need to do anything
                        continue;
                    }
                    index.ix = 0;
                    let mat = mats.get_mut(hand.id()).unwrap();
                    mat.set_ix(index.ix);
                }
                AnimNextState::Some(variant) => {
                    // Transitioning to a new state
                    manager.reset_state(variant.clone());
                }
                AnimNextState::Despawn => {
                    // Triggering the death process for this entity
                    manager.set_hidden(true);
                    *visibility = Visibility::Hidden;
                    commands.entity(manager_eid).despawn_recursive();
                }
            }
        }
        // Update the ix in the manager so it can be read
        progress.ixes.insert(index.body_type, index.ix);
    }
}

pub fn register_anim<StateMachine: AnimStateMachine>(app: &mut App) {
    reg_types!(app, AnimMan<StateMachine>, AnimBodyProgress<StateMachine>);
    app.add_systems(
        PostUpdate,
        (
            handle_manager_changes::<StateMachine>,
            play_animations::<StateMachine>,
        )
            .chain()
            .in_set(AnimationSet)
            .in_set(ManagersSet)
            .after(PhysicsSet),
    );
}
