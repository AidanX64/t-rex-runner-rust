mod app;
mod cloud;
mod collision;
mod config;
mod distance_meter;
mod game_over_panel;
mod horizon;
mod input;
mod night_mode;
mod obstacle;
mod renderer;
mod sprites;
mod trex;

use std::io;
use std::panic;
use std::time::{Duration, Instant};

use app::App;
use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use input::map_event;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

fn main() -> io::Result<()> {
    install_panic_hook();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let result = run(&mut terminal);
    restore_terminal(&mut terminal)?;
    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = App::new();
    let mut last_update = Instant::now();

    loop {
        while event::poll(Duration::ZERO)? {
            let event = event::read()?;
            if let Event::Resize(width, height) = event {
                app.resize(width, height);
            }
            let action = map_event(event);
            app.handle_action(action);
        }

        let now = Instant::now();
        let dt = now.duration_since(last_update);
        if dt >= config::MS_PER_FRAME {
            app.update(dt.as_secs_f64() * 1000.0);
            last_update = now;
        }

        terminal.draw(|frame| renderer::render_app(frame, &app))?;

        if app.quit {
            break;
        }

        std::thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()
}

fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));
}
