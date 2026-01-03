use crate::app::App;
use crate::event::EventHandler;
use crate::ui;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::Terminal;
use ratatui::backend::Backend;

#[derive(Debug, thiserror::Error)]
pub enum TuiError {
    #[error("Failed to enable raw mode")]
    EnableRawMode(std::io::Error),

    #[error("Failed to disable raw mode")]
    DisableRawMode(std::io::Error),

    #[error("Failed to enter alternate screen")]
    EnterAlternateScreen(std::io::Error),

    #[error("Failed to leave alternate screen")]
    LeaveAlternateScreen(std::io::Error),

    #[error("Failed to hide cursor")]
    HideCursor(std::io::Error),

    #[error("Failed to unhide cursor")]
    ShowCursor(std::io::Error),

    #[error("Failed to reset the screen")]
    ResetScreen(std::io::Error),

    #[error("Failed to clear screen")]
    ClearScreen(std::io::Error),

    #[error("UI rendering error: {0}")]
    Rendering(std::io::Error),
}

#[derive(Debug)]
pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    pub fn enter(&mut self) -> Result<(), TuiError> {
        terminal::enable_raw_mode().map_err(|err| TuiError::EnableRawMode(err))?;
        crossterm::execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)
            .map_err(|err| TuiError::EnterAlternateScreen(err))?;

        self.terminal
            .hide_cursor()
            .map_err(|err| TuiError::HideCursor(err))?;
        self.terminal
            .clear()
            .map_err(|err| TuiError::ClearScreen(err))?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> Result<(), TuiError> {
        self.terminal
            .draw(|frame| ui::render_screen(app, frame))
            .map_err(|err| TuiError::Rendering(err))?;
        Ok(())
    }

    fn reset() -> Result<(), TuiError> {
        terminal::disable_raw_mode().map_err(|err| TuiError::DisableRawMode(err))?;
        crossterm::execute!(std::io::stdout(), LeaveAlternateScreen, DisableMouseCapture)
            .map_err(|err| TuiError::LeaveAlternateScreen(err))?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), TuiError> {
        Self::reset()?;
        self.terminal
            .show_cursor()
            .map_err(|err| TuiError::ShowCursor(err))?;
        Ok(())
    }
}

impl<B: ratatui::backend::Backend> Drop for Tui<B> {
    fn drop(&mut self) {
        self.exit().expect("failed to reset terminal");
    }
}
