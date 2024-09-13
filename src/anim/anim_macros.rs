#[macro_export]
macro_rules! defn_animation {
    (
        $name:ident $(,)?
        bodies: [
            $(
                $body_id:ident $(:)? {
                    path: $path:expr,
                    size: ($w:expr, $h:expr),
                    $(
                        length: $length:expr,
                    )?
                    $(
                        fps: $fps:expr,
                    )?
                    $(
                        pos: $pos:expr,
                    )?
                    $(
                        fzix: $fzix:expr,
                    )?
                    $(
                        scale: ($scale_w:expr, $scale_h:expr),
                    )?
                    $(
                        render_layers: $render_layers:expr,
                    )?
                } $(,)?
            )+
        ] $(,)?
        states: [
            $(
                $state_id:ident $(:)? {
                    parts: [
                        $(
                            $part_id:ident$(,)?
                        )+
                    ],
                    $(
                        #[despawn]
                        next: $dummy_despawn:ident,
                    )?
                    $(
                        next: $next_id:ident,
                    )?
                } $(,)?
            )+
        ] $(,)?
    ) => {
        paste::paste! {
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
            #[allow(nonstandard_style)]
            pub enum [<AnimBody_ $name>] {
                $(
                    $body_id,
                )+
            }
            impl Queryable for [<AnimBody_ $name>] {}
            impl AnimBody for [<AnimBody_ $name>] {
                fn to_body_data(&self) -> AnimBodyData {
                    match &self {
                        $(
                            Self::$body_id => {
                                #[allow(unused, unused_mut)]
                                let mut length = 1;
                                #[allow(unused, unused_mut)]
                                let mut fps = DEFAULT_ANIMATION_FPS;
                                #[allow(unused, unused_mut)]
                                let mut pos = Pos::default();
                                #[allow(unused, unused_mut)]
                                let mut fzix = Fzix::default();
                                #[allow(unused, unused_mut)]
                                let mut scale = IVec2::ONE;
                                #[allow(unused, unused_mut)]
                                let mut render_layers = MainLayer::render_layers();

                                $(
                                    length = $length;
                                )?
                                $(
                                    fps = $fps;
                                )?
                                $(
                                    pos = $pos;
                                )?
                                $(
                                    fzix = $fzix;
                                )?
                                $(
                                    scale = IVec2::new($scale_w, $scale_h);
                                )?
                                $(
                                    render_layers = $render_layers;
                                )?

                                AnimBodyData {
                                    path: $path.into(),
                                    size: UVec2::new($w, $h),
                                    length,
                                    fps,
                                    pos,
                                    fzix,
                                    scale,
                                    render_layers,
                                }
                            }
                        )+
                    }
                }
            }

            #[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Reflect)]
            pub enum $name {
                #[default]
                $($state_id,)+
            }
            impl Queryable for $name {}
            impl AnimStateMachine for $name {
                type BodyType = [<AnimBody_ $name>];

                fn to_state_data(&self) -> AnimStateData<Self, Self::BodyType> {
                    match &self {
                        $(
                            Self::$state_id => {
                                let mut overwritten_bodies = vec![];

                                $(
                                    let part_id = Self::BodyType::$part_id;
                                    #[allow(unused, unused_mut)]
                                    let mut overwrite = AnimBodyDataOverrides::default();
                                    overwritten_bodies.push((part_id, overwrite));
                                )+

                                #[allow(unused, unused_mut)]
                                let mut next_state = AnimNextState::None;
                                $(
                                    #[allow(unused)]
                                    let dummy = stringify!($dummy_despawn);
                                    next_state = AnimNextState::Despawn;
                                )?
                                $(
                                    next_state = AnimNextState::Some(Self::$next_id);
                                )?

                                AnimStateData {
                                    overwritten_bodies,
                                    next: next_state,
                                }
                            }
                        )+
                    }
                }
            }
        }
    };
}
pub use defn_animation;
