use super::*;

defn_animation!(
    GBoyAnim,
    bodies: [
        stand: {
            path: "gboy/v1_stand.png",
            size: (17, 17),
            offset: (-0.5, -0.5),
        },
        run: {
            path: "gboy/v1_run.png",
            size: (17, 17),
            length: 5,
            offset: (-0.5, -0.5),
        },
        air_full: {
            path: "gboy/v1_air_full.png",
            size: (17, 17),
            offset: (-0.5, -0.5),
        },
        air_empty: {
            path: "gboy/v1_air_empty.png",
            size: (17, 17),
            length: 3,
            offset: (-0.5, -0.5),
        },
        die: {
            path: "gboy/die1.png",
            size: (32, 32),
            length: 5,
            fps: 24.0,
        },
        empty: {
            path: "empty.png",
            size: (1, 1),
            length: 1,
            fps: 4.0,
        }
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
        AirFull: {
            parts: [
                air_full,
            ],
        },
        AirEmpty: {
            parts: [
                air_empty,
            ],
        },
        Explode: {
            parts: [
                die,
            ],
            next: PostExplodeDelay,
        },
        PostExplodeDelay: {
            parts: [
                empty,
            ],
            #[despawn]
            next: dummy,
        },
    ],
);

defn_animation!(
    RagdollAnim,
    bodies: [
        fall: {
            path: "gboy/v1_die_fall.png",
            size: (17, 17),
            offset: (-0.5, -0.5),
        },
        land: {
            path: "gboy/v1_die_land.png",
            size: (17, 17),
            length: 14,
            offset: (-0.5, -0.5),
        },
        flower_wave: {
            path: "gboy/v1_flower_wave.png",
            size: (17, 17),
            length: 12,
            offset: (-0.5, -0.5),
        },
    ],
    states: [
        Fall: {
            parts: [
                fall,
            ],
        },
        Land: {
            parts: [
                land,
            ],
            next: FlowerWave,
        },
        FlowerWave: {
            parts: [
                flower_wave,
            ],
            #[despawn]
            next: dummy,
        },
    ],
);
