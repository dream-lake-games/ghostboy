use super::*;

defn_animation!(
    DashFadeAnim,
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

defn_animation!(
    SmokeDown,
    bodies: [
        var1: {
            path: "juice/smoke/down1.png",
            size: (16, 16),
            length: 5,
        },
        var2: {
            path: "juice/smoke/down2.png",
            size: (16, 16),
            length: 5,
        },
    ],
    states: [
        Var1 {
            parts: [
                var1,
            ],
            #[despawn]
            next: dummy,
        },
        Var2 {
            parts: [
                var2,
            ],
            #[despawn]
            next: dummy,
        },
    ],
);
impl_rand_variant!(SmokeDown);

defn_animation!(
    SmokeCirc,
    bodies: [
        var1: {
            path: "juice/smoke/circ1.png",
            size: (16, 16),
            length: 4,
        },
        var2: {
            path: "juice/smoke/circ2.png",
            size: (16, 16),
            length: 4,
        },
    ],
    states: [
        Var1 {
            parts: [
                var1,
            ],
            #[despawn]
            next: dummy,
        },
        Var2 {
            parts: [
                var2,
            ],
            #[despawn]
            next: dummy,
        },
    ],
);
impl_rand_variant!(SmokeCirc);
