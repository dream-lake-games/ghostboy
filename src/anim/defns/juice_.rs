use super::*;

defn_animation!(
    DashFade,
    bodies: [
        fade: {
            path: "gboy/v1_dash_fade2.png",
            size: (16, 16),
            length: 2,
        },
    ],
    states: [
        Fade {
            parts: [
                fade,
            ],
            #[despawn]
            next: dummy,
        }
    ],
);
