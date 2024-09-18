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
        fire: {
            path: "enemies/skelly/v1_fire.png",
            size: (16, 18),
            length: 12,
            fps: 12.0,
        },
    ],
    states: [
        Fire: {
            parts: [
                fire,
            ],
            next: Idle,
        },
        Idle {
            parts: [
                idle,
            ],
            next: Fire,
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
