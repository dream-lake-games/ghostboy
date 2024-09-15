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
