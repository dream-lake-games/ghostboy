use super::*;

defn_animation!(
    ButtonAnim,
    bodies: [
        empty: {
            path: "menu/button_empty.png",
            size: (24, 24),
            render_layers: MenuLayer::render_layers(),
        },
        select: {
            path: "menu/button_select.png",
            size: (24, 24),
            length: 2,
            render_layers: MenuLayer::render_layers(),
        },
        selected: {
            path: "menu/button_selected.png",
            size: (24, 24),
            render_layers: MenuLayer::render_layers(),
        }
    ],
    states: [
        Empty {
            parts: [
                empty,
            ],
        },
        Select {
            parts: [
                select,
            ],
            next: Selected,
        },
        Selected {
            parts: [
                selected,
            ],
        },
    ],
);
