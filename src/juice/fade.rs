use crate::prelude::*;

#[derive(Resource, Clone, Debug, Reflect)]
pub struct Fade {
    initial: bool,
    pos: Pos,
    effect: Option<FadeKind>,
}
impl Fade {
    pub fn in_(&mut self, pos: Pos) {
        self.initial = false;
        self.pos = pos;
        self.effect = Some(FadeKind::FadeIn);
    }
    pub fn out(&mut self, pos: Pos) {
        self.initial = false;
        self.pos = pos;
        self.effect = Some(FadeKind::FadeOut);
    }
}

#[derive(Resource, Clone, Debug, Reflect)]
enum FadeKind {
    FadeOut,
    FadeIn,
}
impl FadeKind {
    fn as_state(&self) -> FadeAnim {
        match self {
            Self::FadeIn => FadeAnim::FadeIn,
            Self::FadeOut => FadeAnim::FadeOut,
        }
    }
}

#[derive(Bundle)]
struct FadeBundle {
    name: Name,
    spatial: SpatialBundle,
    anim: AnimMan<FadeAnim>,
}

fn startup_fade(mut commands: Commands, root: Res<TransitionRoot>) {
    let tran = Transform::default().with_scale((Vec2::ONE * 3.0).extend(1.0));
    commands
        .spawn(FadeBundle {
            name: Name::new("fade"),
            spatial: SpatialBundle::from_transform(tran), // NOTE: ZIX handled by the root
            anim: AnimMan::new(),
        })
        .set_parent(root.eid());
}

fn update_fade(
    mut fade: ResMut<Fade>,
    mut anim_q: Query<(&mut AnimMan<FadeAnim>, &mut Transform)>,
    cam_pos_q: Query<&Pos, With<DynamicCamera>>,
) {
    let (mut anim, mut tran) = anim_q.single_mut();
    let cam_pos = cam_pos_q.single();
    if let Some(effect) = fade.effect.clone() {
        anim.set_state(effect.as_state());
        fade.effect = None;
    }
    let diff = fade.pos.as_vec2() - cam_pos.as_vec2();
    if !fade.initial {
        tran.translation = diff.extend(tran.translation.z);
    }
}

pub(super) fn register_fade(app: &mut App) {
    app.insert_resource(Fade {
        initial: true,
        pos: Pos::new(0.0, 0.0),
        effect: None,
    });
    app.add_systems(Startup, startup_fade.after(RootInit));
    app.add_systems(PostUpdate, update_fade.after(CameraSet));
}
