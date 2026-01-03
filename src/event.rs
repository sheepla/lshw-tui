use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use futures::StreamExt;
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Copy)]
pub enum TerminalEvent {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
    receiver: mpsc::Receiver<TerminalEvent>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel(100);

        tokio::spawn(async move {
            let mut reader = event::EventStream::new();

            loop {
                tokio::select! {
                    _ = tokio::time::sleep(tick_rate) => {
                        let _ = sender.try_send(TerminalEvent::Tick);
                    },
                    Some(Ok(event)) = reader.next() => {
                        match event {
                            CrosstermEvent::Key(key) => {
                                let _ = sender.try_send(TerminalEvent::Key(key));
                            }
                            CrosstermEvent::Mouse(mouse) => {
                                let _ = sender.try_send(TerminalEvent::Mouse(mouse));
                            }
                            CrosstermEvent::Resize(width, height) => {
                                let _ = sender.try_send(TerminalEvent::Resize(width, height));
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        Self { receiver }
    }

    pub async fn next(&mut self) -> Option<TerminalEvent> {
        self.receiver.recv().await
    }
}