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

defn_animation!(
    FadeAnim,
    bodies: [
        clear: {
            path: "juice/clear.png",
            size: (160, 144),
            render_layers: MenuLayer::render_layers(),
        },
        black: {
            path: "juice/black.png",
            size: (160, 144),
            render_layers: MenuLayer::render_layers(),
        },
        fade_in: {
            path: "juice/fade_in.png",
            size: (160, 144),
            length: 11,
            fps: 36.0,
            render_layers: MenuLayer::render_layers(),
        },
        fade_out: {
            path: "juice/fade_out.png",
            size: (160, 144),
            length: 11,
            fps: 36.0,
            render_layers: MenuLayer::render_layers(),
        },
    ],
    states: [
        Clear {
            parts: [
                clear,
            ],
        },
        Black {
            parts: [
                black,
            ],
        },

        FadeIn {
            parts: [
                fade_in,
            ],
            next: Clear,
        },
        FadeOut {
            parts: [
                fade_out,
            ],
            next: Black,
        },
    ],
);

defn_animation!(
    RainAnim,
    bodies: [
        steady: {
            path: "environment/rain.png",
            size: (16, 16),
            length: 32,
            fps: 160.0,
            // Don't overthink. This works.
            scale: (1000, 1000),
            reps: (1000, 1000),
        }
    ],
    states: [
        Steady {
            parts: [
                steady,
            ],
        },
    ],
);

defn_animation!(
    LightningAnim,
    bodies: [
        var1: {
            path: "environment/storm/light1.png",
            size: (160, 144),
            render_layers: BgLayer::render_layers(),
        },
        var2: {
            path: "environment/storm/light2.png",
            size: (160, 144),
            render_layers: BgLayer::render_layers(),
        },
        var3: {
            path: "environment/storm/light3.png",
            size: (160, 144),
            render_layers: BgLayer::render_layers(),
        },
        var4: {
            path: "environment/storm/light4.png",
            size: (160, 144),
            render_layers: BgLayer::render_layers(),
        },
    ],
    states: [
        Var1 {
            parts: [
                var1,
            ],
        },
        Var2 {
            parts: [
                var2,
            ],
        },
        Var3 {
            parts: [
                var3,
            ],
        },
        Var4 {
            parts: [
                var4,
            ],
        },
    ],
);
impl_rand_variant!(LightningAnim);
