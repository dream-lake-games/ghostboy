use super::*;

defn_animation!(
    TombstoneAnim,
    bodies: [
        active: {
            path: "environment/tombstone/active.png",
            size: (24, 24),
        },
        empty: {
            path: "environment/tombstone/empty.png",
            size: (24, 24),
        },
        inactive: {
            path: "environment/tombstone/inactive.png",
            size: (24, 24),
        },
        reach: {
            path: "environment/tombstone/reach.png",
            size: (24, 24),
            length: 4,
        },
    ],
    states: [
        // Remember the first one is default
        Empty {
            parts: [
                empty,
            ],
        }
        Active {
            parts: [
                active,
            ],
        }
        Inactive {
            parts: [
                inactive,
            ],
        }
        Reach {
            parts: [
                reach,
            ],
            next: Active,
        }
    ],
);
