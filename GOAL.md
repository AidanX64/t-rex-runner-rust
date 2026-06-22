# GOAL: Port Chrome Dino Runner to Rust TUI

Port the extracted Chrome offline T-Rex runner game (`index.js`, 2752 lines) to a cross-platform Rust terminal application using `ratatui` + `crossterm` with block-character rendering.

## Source

- `index.js` — Single-file JS game (Chromium source extract)
- `index.html` + `index.css` — Original DOM/HTML wrapper (discarded — no browser)
- `assets/` — PNG spritesheets and GIF screenshots (discarded — block-art replaces sprites)
- `LICENSE` — BSD-3-Clause (inherited)

## Scope

### Core Game (FULL parity required)

| Feature | Original | Rust TUI |
|---|---|---|
| T-Rex with 5 states (WAITING/RUNNING/JUMPING/DUCKING/CRASHED) | ✅ | ✅ |
| Jump physics (gravity, velocity, min/max height, speed drop) | ✅ | ✅ |
| Duck while running / speed drop while jumping | ✅ | ✅ |
| T-Rex blink animation at random intervals | ✅ | ✅ |
| Cactus obstacles (small: 17×35, large: 25×50) | ✅ | ✅ |
| Pterodactyl obstacles (2 animation frames, 3 heights) | ✅ | ✅ |
| Obstacle gap calculation scaling with speed | ✅ | ✅ |
| Max obstacle duplication prevention | ✅ | ✅ |
| AABB collision detection with multi-box (6 t-rex boxes, 3-5 obstacle boxes) | ✅ | ✅ |
| Ground scrolling (2-segment, bump variation) | ✅ | ✅ |
| Parallax clouds (random height, frequency, max 6) | ✅ | ✅ |
| Night mode (moon phases, stars, invert every 700 units, 12s fade) | ✅ | ✅ |
| Distance/score meter (5-digit, achievement flash every 100 units) | ✅ | ✅ |
| High score display (HI prefix, persistent per session) | ✅ | ✅ |
| Game over panel (text + restart button centered) | ✅ | ✅ |
| Speed acceleration (6 → 13, +0.001/frame) | ✅ | ✅ |
| Arcade mode scaling (responsive to terminal size) | ✅ | ✅ |
| Pause on terminal blur (app backgrounded) | ✅ | ✅ |

### Input

| Action | Key |
|---|---|
| Jump / Start | Space, ↑ |
| Duck / Speed drop | ↓ |
| Restart | Enter, Space (after delay), click |

### Rendering

- All sprites defined as block-character matrices in `sprites.rs`
- No external image files — everything is inline Rust const arrays
- Use Unicode block characters (`█▀▄▌▐▖▗▘▝▚▞▙▟░▒▓ `) for 2×2 pixel-per-cell resolution
- Match original sprite dimensions exactly (pixel → block mapping)

### Non-Goals (intentionally excluded)

| Original Feature | Reason Dropped |
|---|---|
| Audio (Web Audio API / base64 MP3) | No audio; terminal-only |
| HiDPI sprite variants | Terminal has fixed font resolution |
| DOM/CSS animations | No browser; pure terminal |
| Touch controller overlay | Not applicable to terminal |
| `loadTimeData` / disabled easter egg | Chrome-specific; not relevant |
| PNG spritesheets | Replaced with block-art |

## Success Criteria

1. Game boots in any terminal with a blinking T-Rex (WAITING state)
2. Pressing Space/↑ makes the T-Rex jump and starts the game
3. Obstacles spawn from the right, scroll left, and vanish
4. Collision with an obstacle ends the game (CRASHED state)
5. Game over panel appears with restart functionality
6. Distance/score increases over time with acceleration
7. Night mode triggers periodically with moon and stars
8. High score persists across restarts within a session
9. Window resize is handled gracefully
10. All physics values match original gameplay feel

## Crate Structure

```
t-rex-runner/
├── Cargo.toml
└── src/
    ├── main.rs              Entry, terminal init, game loop
    ├── app.rs               Game state machine (Runner equivalent)
    ├── trex.rs              T-Rex: states, physics, jump/duck
    ├── obstacle.rs          Cactus + Pterodactyl types, spawning, gap logic
    ├── collision.rs         CollisionBox, AABB multi-box detection
    ├── horizon.rs           Horizon: ground scrolling, clouds, night mode
    ├── cloud.rs             Cloud behavior
    ├── night_mode.rs        Moon phases, star placement, fade
    ├── distance_meter.rs    Score digits, high score, achievement flash
    ├── game_over_panel.rs   Game over text + restart button
    ├── renderer.rs          Block-art rendering to ratatui Canvas
    ├── sprites.rs           All sprite block-matrix definitions
    ├── config.rs            Game constants (physics, speeds, sizes)
    └── input.rs             Crossterm keyboard/mouse/touch mapping
```

## Dependencies

```toml
[dependencies]
ratatui = "0.29"
crossterm = "0.28"
rand = "0.8"
```

## License

BSD-3-Clause (matching original LICENSE file).
