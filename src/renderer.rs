use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::canvas::{Canvas, Context};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::App;
use crate::config;
use crate::sprites::{self, Sprite};

pub fn render_app(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let fg = if app.horizon.night_mode.opacity > 0.5 {
        Color::White
    } else {
        Color::Gray
    };

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::NONE))
        .x_bounds([0.0, config::DEFAULT_WIDTH])
        .y_bounds([0.0, config::DEFAULT_HEIGHT])
        .paint(|ctx| {
            draw_night_mode(ctx, app, fg);
            draw_clouds(ctx, app, fg);
            draw_horizon(ctx, app, fg);
            draw_obstacles(ctx, app, fg);
            draw_trex(ctx, app, fg);
            draw_score(ctx, app, fg);
            if app.game_over_panel.visible {
                draw_game_over(ctx, app, fg);
            }
            if app.paused {
                draw_center_text(ctx, "PAUSED", 70.0, fg);
            }
        });

    frame.render_widget(canvas, area);
    if area.width < 45 || area.height < 12 {
        render_tiny_warning(frame, area);
    }
}

fn draw_trex(ctx: &mut Context, app: &App, color: Color) {
    draw_sprite(
        ctx,
        sprites::get(app.trex.sprite_key()),
        app.trex.x_pos,
        app.trex.y_pos,
        color,
    );
}

fn draw_obstacles(ctx: &mut Context, app: &App, color: Color) {
    for obstacle in &app.horizon.obstacles {
        draw_sprite(
            ctx,
            sprites::get(obstacle.sprite_key()),
            obstacle.x_pos,
            obstacle.y_pos,
            color,
        );
    }
}

fn draw_clouds(ctx: &mut Context, app: &App, color: Color) {
    for cloud in &app.horizon.clouds {
        draw_sprite(ctx, sprites::get("cloud"), cloud.x_pos, cloud.y_pos, color);
    }
}

fn draw_horizon(ctx: &mut Context, app: &App, color: Color) {
    for x in app.horizon.horizon_line.x_pos {
        let mut px = x;
        while px < x + config::HORIZON_WIDTH {
            let y = world_y(config::HORIZON_YPOS);
            let ch = if ((px as i32) / 18) % 7 == 0 {
                "▄"
            } else {
                "▀"
            };
            ctx.print(px, y, Span::styled(ch, Style::default().fg(color)));
            px += 6.0;
        }
    }
}

fn draw_night_mode(ctx: &mut Context, app: &App, color: Color) {
    let night = &app.horizon.night_mode;
    if night.opacity <= 0.0 {
        return;
    }

    for star in &night.stars {
        ctx.print(
            star.x,
            world_y(star.y),
            Span::styled("*", Style::default().fg(color)),
        );
    }

    let phase = match night.current_phase {
        0 => "●",
        1 | 6 => "◕",
        2 | 5 => "◐",
        _ => "○",
    };
    ctx.print(
        night.x_pos,
        world_y(night.y_pos),
        Span::styled(phase, Style::default().fg(color)),
    );
}

fn draw_score(ctx: &mut Context, app: &App, color: Color) {
    if app.distance_meter.high_score > 0 {
        ctx.print(
            420.0,
            world_y(8.0),
            Span::styled(
                app.distance_meter.high_score_string(),
                Style::default().fg(color),
            ),
        );
    }
    if !app.distance_meter.achievement || ((app.distance_meter.distance / 10) & 1) == 0 {
        ctx.print(
            540.0,
            world_y(8.0),
            Span::styled(
                app.distance_meter.distance_string(),
                Style::default().fg(color),
            ),
        );
    }
}

fn draw_game_over(ctx: &mut Context, app: &App, color: Color) {
    draw_center_text(ctx, "GAME OVER", 55.0, color);
    let label = if app.can_restart() {
        "[ ENTER ]"
    } else {
        "..."
    };
    draw_center_text(ctx, label, 78.0, color);
}

fn draw_center_text(ctx: &mut Context, text: &str, y: f64, color: Color) {
    let x = config::DEFAULT_WIDTH / 2.0 - text.len() as f64 * 4.0;
    ctx.print(
        x,
        world_y(y),
        Span::styled(text.to_string(), Style::default().fg(color)),
    );
}

fn draw_sprite(ctx: &mut Context, sprite: &Sprite, x: f64, y: f64, color: Color) {
    let row_step = sprite.height / sprite.rows.len().max(1) as f64;
    for (row_idx, row) in sprite.rows.iter().enumerate() {
        let col_count = row.chars().count().max(1) as f64;
        let col_step = sprite.width / col_count;
        for (col_idx, ch) in row.chars().enumerate() {
            if ch != ' ' {
                ctx.print(
                    x + col_idx as f64 * col_step,
                    world_y(y + row_idx as f64 * row_step),
                    Span::styled(ch.to_string(), Style::default().fg(color)),
                );
            }
        }
    }
}

fn world_y(y: f64) -> f64 {
    config::DEFAULT_HEIGHT - y
}

fn render_tiny_warning(frame: &mut Frame, area: Rect) {
    let text = Paragraph::new("Resize terminal for T-Rex Runner")
        .style(Style::default().fg(Color::Yellow));
    frame.render_widget(text, area);
}
