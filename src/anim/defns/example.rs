use super::*;

defn_animation!(
    SuicidoBody,
    bodies: [
        charge: {
            path: "enemies/suicido/charge.png",
            size: (16, 16),
            length: 4,
            fps: 16.0,
        },
        launch: {
            path: "enemies/suicido/launch.png",
            size: (16, 16),
            length: 4,
            fps: 16.0,
        },
        explode: {
            path: "enemies/suicido/explode.png",
            size: (16, 16),
            length: 4,
            fps: 8.0,
        },
        warning_circle: {
            path: "enemies/suicido/warning_circle.png",
            size: (64, 64),
            length: 3,
            fps: 18.0,
        },
    ],
    states: [
        Charge: {
            parts: [
                charge,
            ],
        },
        Launch: {
            parts: [
                launch,
            ],
        },
        Explode: {
            parts: [
                explode,
                warning_circle,
            ],
            #[despawn]
            next: dummy,
        }
    ],
);

defn_animation!(
    SuicidoExplosion,
    bodies: [
        core: {
            path: "enemies/suicido/explosion_circle.png",
            size: (64, 64),
            length: 4,
            fps: 12.0,
        },
    ],
    states: [
        Charge: {
            parts: [
                core,
            ],
            #[despawn]
            next: dummy,
        },
    ],
);
