use super::*;

defn_animation!(
    SkellyAnim,
    bodies: [
        idle: {
            path: "enemies/skelly/v1_idle.png",
            size: (16, 18),
            length: 3,
            fps: 4.0,
        },
        fire_e: {
            path: "enemies/skelly/v1_fire_e.png",
            size: (16, 18),
            length: 12,
            fps: 12.0,
        },
        fire_n: {
            path: "enemies/skelly/v1_fire_n.png",
            size: (16, 18),
            length: 12,
            fps: 12.0,
        },
        fire_ne: {
            path: "enemies/skelly/v1_fire_ne.png",
            size: (16, 18),
            length: 12,
            fps: 12.0,
        },
        fire_se: {
            path: "enemies/skelly/v1_fire_se.png",
            size: (16, 18),
            length: 12,
            fps: 12.0,
        },
        ref_: {
            path: "enemies/skelly/v1_ref.png",
            size: (16, 18),
        },
    ],
    states: [
        FireE: {
            parts: [
                fire_e,
            ],
            next: Idle,
        },
        FireN: {
            parts: [
                fire_n,
            ],
            next: Idle,
        },
        FireNE: {
            parts: [
                fire_ne,
            ],
            next: Idle,
        },
        FireSE: {
            parts: [
                fire_se,
            ],
            next: Idle,
        },
        Idle {
            parts: [
                idle,
            ],
            next: Restart,
        }
        Restart {
            parts: [
                ref_,
            ],
        }
    ],
);

defn_animation!(
    ArrowAnim,
    bodies: [
        north: {
            path: "enemies/skelly/arrow_north.png",
            size: (9, 9),
            offset: (-0.5, -0.5),
        },
        northeast: {
            path: "enemies/skelly/arrow_northeast.png",
            size: (9, 9),
            offset: (-0.5, -0.5),
        },
        east: {
            path: "enemies/skelly/arrow_east.png",
            size: (9, 9),
            offset: (-0.5, -0.5),
        },
        north_fade: {
            path: "enemies/skelly/arrow_north_fade.png",
            size: (9, 9),
            length: 3,
            offset: (-0.5, -0.5),
        },
        northeast_fade: {
            path: "enemies/skelly/arrow_northeast_fade.png",
            size: (9, 9),
            length: 3,
            offset: (-0.5, -0.5),
        },
        east_fade: {
            path: "enemies/skelly/arrow_east_fade.png",
            size: (9, 9),
            length: 3,
            offset: (-0.5, -0.5),
        },
    ],
    states: [
        North: {
            parts: [
                north,
            ],
        },
        NorthEast {
            parts: [
                northeast,
            ],
        }
        East {
            parts: [
                east,
            ],
        }
        NorthFade: {
            parts: [
                north_fade,
            ],
            #[despawn]
            next: dummy,
        },
        NorthEastFade {
            parts: [
                northeast_fade,
            ],
            #[despawn]
            next: dummy,
        }
        EastFade {
            parts: [
                east_fade,
            ],
            #[despawn]
            next: dummy,
        }
    ],
);
