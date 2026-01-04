use crate::app::App;
use crate::event::EventHandler;
use crate::ui;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::Backend;
use ratatui::Terminal;
use tracing::error;

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
        terminal::enable_raw_mode().map_err(TuiError::EnableRawMode)?;
        crossterm::execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)
            .map_err(TuiError::EnterAlternateScreen)?;

        self.terminal
            .hide_cursor()
            .map_err(TuiError::HideCursor)?;
        self.terminal.clear().map_err(TuiError::ClearScreen)?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> Result<(), TuiError> {
        self.terminal
            .draw(|frame| ui::render_screen(app, frame))
            .map_err(TuiError::Rendering)?;
        Ok(())
    }

    fn reset() -> Result<(), TuiError> {
        terminal::disable_raw_mode().map_err(TuiError::DisableRawMode)?;
        crossterm::execute!(std::io::stdout(), LeaveAlternateScreen, DisableMouseCapture)
            .map_err(TuiError::LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), TuiError> {
        Self::reset()?;
        self.terminal.show_cursor().map_err(TuiError::ShowCursor)?;
        Ok(())
    }
}

impl<B: ratatui::backend::Backend> Drop for Tui<B> {
    fn drop(&mut self) {
        if let Err(e) = self.exit() {
            error!("failed to reset terminal: {}", e);
        }
    }
}