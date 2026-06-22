# AGENTS.md — T-Rex Runner Rust TUI Port

This file guides codex/opencode agents working on porting the Chrome Dino game from JavaScript to a Rust terminal UI application.

## Agent Instructions

When modifying code, always:
- Read the GOAL.md first to understand the full scope
- Read this file for crate structure, conventions, and porting rules
- Never add comments unless the original JS had them (and even then, only as needed)
- Mimic the existing Rust file conventions in the project
- Keep physics and game state logic identical to the original JS — rendering is the only layer that changes
- Don't change or write to the original JS code, only read the JS code

## Crate Structure

```
src/
├── main.rs              Entry point, terminal init, game loop
├── app.rs               App state machine (Runner equivalent)
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

No audio. No external assets. Everything is inline block-art matrices.

## Porting Rules

### Game State → 1:1
The original JS game logic must port directly. Only the rendering and input layers change:
- `drawImage()` → `renderer::draw_sprite(canvas, x, y, sprite_data)`
- DOM events → crossterm `event::poll` / `event::read`
- `requestAnimationFrame` → fixed-timestep loop in `main.rs`
- Canvas pixel coords → terminal block-character cells

### Sprite Definitions
Do NOT load the PNG spritesheet. Instead, define every sprite as a block-character matrix in `sprites.rs`:
- Each sprite is a `Vec<Vec<&str>>` or `[[&str; W]; H]` at block level
- Use `█`, `▀`, `▄`, `▌`, `▐`, `▖`, `▗`, `▘`, `▝`, `▚`, `▞`, `▙`, `▟`, `░`, `▒`, `▓`, ` ` to achieve 2x2 pixel resolution per terminal cell
- Match the original sprite dimensions exactly (e.g., T-Rex is 44x47 pixels = 22x47 block chars)

### Config Values
All constants from the JS (speeds, gravity, gap coefficients, etc.) go in `config.rs` as `const` or `struct Config`. Keep the exact same numeric values.

### Collision Detection
Port `CollisionBox`, `boxCompare`, `createAdjustedCollisionBox`, and `checkForCollision` identically. The collision boxes are pixel-based, not block-based — use the same x,y,w,h values from the JS.

### Rendering Layer
- `renderer.rs` takes the game state and draws everything to a `ratatui::widgets::Canvas`
- Each game element (t-rex, obstacles, ground, clouds, moon, score, game over) has a draw function
- Use `canvas.print()` / `canvas.draw()` with block characters
- The canvas dimensions should map to the original 600x150 pixel space, scaled to terminal size

### Game Loop (main.rs)
```
loop {
    // Input (non-blocking poll)
    while crossterm::event::poll(Duration::ZERO) {
        handle_input(event, &mut app)
    }

    // Fixed timestep update
    let now = Instant::now();
    let dt = now - last_update;
    if dt >= MS_PER_FRAME {
        app.update(dt);
        last_update = now;
    }

    // Render
    terminal.draw(|frame| render_app(frame, &app));
}
```

### Naming Conventions
- Types: `PascalCase` — `Trex`, `Obstacle`, `CollisionBox`, `DistanceMeter`
- Functions + methods: `snake_case` — `start_jump`, `check_collision`
- Constants: `SCREAMING_SNAKE_CASE` — `MAX_SPEED`, `GRAVITY`
- Struct fields: `snake_case`

## Verification

After any change, run:
```
cargo check
cargo run
```

Test all game states: idle (blinking), running, jumping, ducking, speed drop, collision, game over, restart, night mode, high score, resize.
