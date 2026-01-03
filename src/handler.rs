use crate::{action::Action, state::State};
use crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseEventKind};

pub fn handle_key_events(key_event: KeyEvent, _state: &State) -> Option<Action> {
    match key_event.code {
        KeyCode::Tab => Some(Action::SwitchFocus),
        KeyCode::Up => Some(Action::Up),
        KeyCode::Down => Some(Action::Down),
        KeyCode::Left => Some(Action::CloseNode),
        KeyCode::Right => Some(Action::OpenNode),
        KeyCode::Char('k') => Some(Action::Up),
        KeyCode::Char('j') => Some(Action::Down),
        KeyCode::Char('h') => Some(Action::CloseNode),
        KeyCode::Char('l') => Some(Action::OpenNode),
        KeyCode::Enter => Some(Action::OpenNode),
        KeyCode::Backspace => Some(Action::CloseNode),
        KeyCode::Char('r') => Some(Action::Reload),
        KeyCode::Char('q') => Some(Action::Quit),
        _ => None,
    }
}

pub fn handle_mouse_events(mouse_event: &MouseEvent, _state: &State) -> Option<Action> {
    match mouse_event.kind {
        MouseEventKind::ScrollDown => Some(Action::Down),
        MouseEventKind::ScrollUp => Some(Action::Up),
        _ => None,
    }
}
