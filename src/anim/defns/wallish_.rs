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
