use super::*;

defn_animation!(
    GBoyAnim,
    bodies: [
        stand: {
            path: "gboy/v1_stand.png",
            size: (18, 18),
            offset: (-0.5, -0.5),
        },
        run: {
            path: "gboy/v1_run.png",
            size: (17, 17),
            length: 5,
            offset: (-0.5, -0.5),
        },
        air: {
            path: "gboy/v1_air.png",
            size: (17, 17),
            offset: (-0.5, -0.5),
        },
    ],
    states: [
        Stand: {
            parts: [
                stand,
            ],
        },
        Run: {
            parts: [
                run,
            ],
        },
        Air: {
            parts: [
                air,
            ],
        }
    ],
);
