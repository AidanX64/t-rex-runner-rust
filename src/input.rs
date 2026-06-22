use crossterm::event::{Event, KeyCode, KeyEventKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputAction {
    JumpPressed,
    JumpReleased,
    DuckPressed,
    DuckReleased,
    Restart,
    Pause,
    Resume,
    Quit,
    None,
}

pub fn map_event(event: Event) -> InputAction {
    match event {
        Event::Key(key) => match (key.code, key.kind) {
            (KeyCode::Char('q') | KeyCode::Esc, KeyEventKind::Press) => InputAction::Quit,
            (KeyCode::Char('p'), KeyEventKind::Press) => InputAction::Pause,
            (KeyCode::Char('r'), KeyEventKind::Press) => InputAction::Resume,
            (KeyCode::Enter, KeyEventKind::Press) => InputAction::Restart,
            (KeyCode::Char(' ') | KeyCode::Up, KeyEventKind::Press) => InputAction::JumpPressed,
            (KeyCode::Char(' ') | KeyCode::Up, KeyEventKind::Release) => InputAction::JumpReleased,
            (KeyCode::Down, KeyEventKind::Press) => InputAction::DuckPressed,
            (KeyCode::Down, KeyEventKind::Release) => InputAction::DuckReleased,
            _ => InputAction::None,
        },
        Event::FocusLost => InputAction::Pause,
        Event::FocusGained => InputAction::Resume,
        Event::Mouse(_) => InputAction::Restart,
        _ => InputAction::None,
    }
}
