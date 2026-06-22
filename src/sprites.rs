pub struct Sprite {
    pub width: f64,
    pub height: f64,
    pub rows: &'static [&'static str],
}

pub const TREX: Sprite = Sprite {
    width: 44.0,
    height: 47.0,
    rows: &[
        "        ██████  ",
        "       ████████ ",
        "       ████ ███ ",
        "       ████████ ",
        "█      ██████   ",
        "█     ████████  ",
        "██  ███████     ",
        "███████████     ",
        " █████████      ",
        "  ███████       ",
        "   █  █         ",
        "   █  █         ",
        "   ██ ██        ",
    ],
};

pub const TREX_BLINK: Sprite = Sprite {
    rows: TREX.rows,
    ..TREX
};
pub const TREX_CRASHED: Sprite = Sprite {
    rows: &[
        "        ██████  ",
        "       ███x███ ",
        "       ███████ ",
        "       ███████ ",
        "█      ██████  ",
        "█     ████████ ",
        "██  ███████    ",
        "███████████    ",
        " █████████     ",
        "  ███████      ",
        "   █  █        ",
        "   █  █        ",
        "   ██ ██       ",
    ],
    ..TREX
};
pub const TREX_RUN_1: Sprite = Sprite {
    rows: TREX.rows,
    ..TREX
};
pub const TREX_RUN_2: Sprite = Sprite {
    rows: &[
        "        ██████  ",
        "       ████████ ",
        "       ████ ███ ",
        "       ████████ ",
        "█      ██████   ",
        "█     ████████  ",
        "██  ███████     ",
        "███████████     ",
        " █████████      ",
        "  ███████       ",
        "    █ █         ",
        "    █ █         ",
        "   ██ ██        ",
    ],
    ..TREX
};
pub const TREX_DUCK_1: Sprite = Sprite {
    width: 59.0,
    height: 25.0,
    rows: &[
        "       ████████████  ",
        "  ██████████████████ ",
        "███████████████ ████ ",
        " ██████████████████  ",
        "   ██████████████    ",
        "     ██      ██      ",
        "     ██      ██      ",
    ],
};
pub const TREX_DUCK_2: Sprite = Sprite {
    rows: &[
        "       ████████████  ",
        "  ██████████████████ ",
        "███████████████ ████ ",
        " ██████████████████  ",
        "   ██████████████    ",
        "      ██    ██       ",
        "      ██    ██       ",
    ],
    ..TREX_DUCK_1
};

pub const CACTUS_SMALL_1: Sprite = Sprite {
    width: 17.0,
    height: 35.0,
    rows: &[
        "   ██   ",
        "█  ██   ",
        "█  ██ █ ",
        "█████ █ ",
        "   ████ ",
        "   ██   ",
        "   ██   ",
        "   ██   ",
    ],
};
pub const CACTUS_SMALL_2: Sprite = Sprite {
    width: 34.0,
    height: 35.0,
    rows: &[
        "   ██      ██   ",
        "█  ██   █  ██   ",
        "█  ██ █ █  ██ █ ",
        "█████ █ █████ █ ",
        "   ████    ████ ",
        "   ██      ██   ",
        "   ██      ██   ",
        "   ██      ██   ",
    ],
};
pub const CACTUS_SMALL_3: Sprite = Sprite {
    width: 51.0,
    height: 35.0,
    rows: &[
        "   ██      ██      ██   ",
        "█  ██   █  ██   █  ██   ",
        "█  ██ █ █  ██ █ █  ██ █ ",
        "█████ █ █████ █ █████ █ ",
        "   ████    ████    ████ ",
        "   ██      ██      ██   ",
        "   ██      ██      ██   ",
        "   ██      ██      ██   ",
    ],
};
pub const CACTUS_LARGE_1: Sprite = Sprite {
    width: 25.0,
    height: 50.0,
    rows: &[
        "    ███    ",
        "    ███    ",
        "█   ███    ",
        "█   ███  █ ",
        "██  ███  █ ",
        "███████  █ ",
        "    ██████ ",
        "    ███    ",
        "    ███    ",
        "    ███    ",
        "    ███    ",
    ],
};
pub const CACTUS_LARGE_2: Sprite = Sprite {
    width: 50.0,
    height: 50.0,
    rows: &[
        "    ███        ███    ",
        "    ███        ███    ",
        "█   ███    █   ███    ",
        "█   ███  █ █   ███  █ ",
        "██  ███  █ ██  ███  █ ",
        "███████  █ ███████  █ ",
        "    ██████     ██████ ",
        "    ███        ███    ",
        "    ███        ███    ",
        "    ███        ███    ",
        "    ███        ███    ",
    ],
};
pub const CACTUS_LARGE_3: Sprite = Sprite {
    width: 75.0,
    height: 50.0,
    rows: &[
        "    ███        ███        ███    ",
        "    ███        ███        ███    ",
        "█   ███    █   ███    █   ███    ",
        "█   ███  █ █   ███  █ █   ███  █ ",
        "██  ███  █ ██  ███  █ ██  ███  █ ",
        "███████  █ ███████  █ ███████  █ ",
        "    ██████     ██████     ██████ ",
        "    ███        ███        ███    ",
        "    ███        ███        ███    ",
        "    ███        ███        ███    ",
        "    ███        ███        ███    ",
    ],
};
pub const PTERODACTYL_1: Sprite = Sprite {
    width: 46.0,
    height: 40.0,
    rows: &[
        "      ██        ",
        "     ████       ",
        "███████████     ",
        "  ████████████  ",
        "    ███████  ██ ",
        "      ███       ",
        "     ██ ██      ",
    ],
};
pub const PTERODACTYL_2: Sprite = Sprite {
    rows: &[
        "      ██        ",
        "██   ████   ██  ",
        " ████████████   ",
        "   █████████    ",
        "     █████      ",
        "      ███       ",
        "     ██ ██      ",
    ],
    ..PTERODACTYL_1
};
pub const CLOUD: Sprite = Sprite {
    width: 46.0,
    height: 14.0,
    rows: &[
        "    █████       ",
        "  ██████████    ",
        "██████████████  ",
        " ██████████████ ",
    ],
};

pub fn get(key: &str) -> &'static Sprite {
    match key {
        "trex_blink" => &TREX_BLINK,
        "trex_crashed" => &TREX_CRASHED,
        "trex_run_1" => &TREX_RUN_1,
        "trex_run_2" => &TREX_RUN_2,
        "trex_duck_1" => &TREX_DUCK_1,
        "trex_duck_2" => &TREX_DUCK_2,
        "cactus_small_1" => &CACTUS_SMALL_1,
        "cactus_small_2" => &CACTUS_SMALL_2,
        "cactus_small_3" => &CACTUS_SMALL_3,
        "cactus_large_1" => &CACTUS_LARGE_1,
        "cactus_large_2" => &CACTUS_LARGE_2,
        "cactus_large_3" => &CACTUS_LARGE_3,
        "pterodactyl_1" => &PTERODACTYL_1,
        "pterodactyl_2" => &PTERODACTYL_2,
        "cloud" => &CLOUD,
        _ => &TREX,
    }
}
