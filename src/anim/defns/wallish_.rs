use super::*;

defn_animation!(
    EphPlatAnim,
    bodies: [
        solid: {
            path: "environment/ephplat_ref.png",
            size: (8, 8),
        },
        fade: {
            path: "environment/ephplat_fade.png",
            size: (8, 8),
            length: 5,
        },
        refill: {
            path: "environment/ephplat_refill.png",
            size: (8, 8),
            length: 8,
            fps: 8.0,
        },
    ],
    states: [
        Solid {
            parts: [
                solid,
            ],
        }
        Fade {
            parts: [
                fade,
            ],
            next: Refill,
        }
        Refill {
            parts: [
                refill,
            ],
            next: Solid,
        }
    ],
);

defn_animation!(
    GuilloAnim,
    bodies: [
        up: {
            path: "environment/guillo_up.png",
            size: (16, 36),
        },
        fall: {
            path: "environment/guillo_fall.png",
            size: (16, 36),
            length: 6,
        },
        down: {
            path: "environment/guillo_down.png",
            size: (16, 36),
        },
    ],
    states: [
        Up {
            parts: [
                up,
            ],
        }
        Fall {
            parts: [
                fall,
            ],
            next: Down,
        }
        Down {
            parts: [
                down,
            ],
        }
    ],
);
