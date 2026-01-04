use action::Action;
use clap::Parser;
use dispatcher::Dispatcher;
use tokio::sync::mpsc;
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{
    app::App,
    event::{EventHandler, TerminalEvent},
    handler::{handle_key_events, handle_mouse_events},
    tui::Tui,
};

// Add tracing imports
use tracing_subscriber::{self, filter::EnvFilter};
use tracing_subscriber::fmt::time::UtcTime;
use std::fs; // For creating log directory
use std::path::Path; // For log directory path

pub mod action;
pub mod app;
pub mod core;
pub mod debug;
pub mod dispatcher;
pub mod event;
pub mod handler;
pub mod state;
pub mod tui;
pub mod ui;
pub mod widgets;

#[derive(Debug, clap::Parser)]
struct Args {
    /// Command name of 'lshw'
    #[arg(short, long, default_value = "lshw")]
    command: String,

    /// Sanitize sensitive values like serial numbers, etc.
    #[arg(short, long, default_value_t = false)]
    sanitize: bool,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = Args::parse();

    // Initialize tracing_subscriber conditionally
    #[cfg(debug_assertions)]
    {
        let log_dir = Path::new("log");
        if !log_dir.exists() {
            let _ = fs::create_dir(log_dir);
        }
        let log_file = fs::File::create(log_dir.join("debug.log"))?;

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env().add_directive("lshw_tui=debug".parse()?))
            .with_writer(log_file)
            .with_ansi(false) // Disable ANSI colors for file output
            .with_timer(UtcTime::new(time::macros::format_description!("[hour]:[minute]:[second].[subsecond]")))
            .init();
    }

    color_eyre::install()?;

    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(10);
    let mut tui = Tui::new(terminal, events);
    let (action_sender, mut action_receiver) = mpsc::unbounded_channel::<Action>();
    let dispatcher = Dispatcher::new(action_sender);

    tui.enter()?;

    let mut app = App::init(args.command.as_str(), args.sanitize).await?;

    while !app.state.should_quit {
        tokio::select! {
            Some(action) = action_receiver.recv() => {
                dispatcher.dispatch(&mut app, action).await;
            }

            maybe_event = tui.events.next() => {
                if let Some(event) = maybe_event {
                    match event {
                        TerminalEvent::Tick => {
                            app.tick();
                        }
                        TerminalEvent::Key(key_event) => {
                            if let Some(action) = handle_key_events(key_event, &app.state) {
                                dispatcher.dispatch(&mut app, action).await;
                            }
                        }
                        TerminalEvent::Mouse(mouse_event) => {
                            if let Some(action) = handle_mouse_events(&mouse_event, &app.state) {
                                dispatcher.dispatch(&mut app, action).await;
                            }
                        }
                        TerminalEvent::Resize(_, _) => {}
                    }
                }
            }
        }
        tui.draw(&mut app)?;
    }

    Ok(())
}
