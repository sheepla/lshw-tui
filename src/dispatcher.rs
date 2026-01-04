use tokio::sync::mpsc::UnboundedSender;
use tracing::error;

use crate::{action::Action, app::App, state::WidgetFocus};

#[derive(Debug)]
pub struct Dispatcher {
    sender: UnboundedSender<Action>,
}

impl Dispatcher {
    pub fn new(sender: UnboundedSender<Action>) -> Self {
        Self { sender }
    }

    pub async fn dispatch(&self, app: &mut App, action: Action) {
        let state = &mut app.state;
        match state.widget_focus {
            WidgetFocus::TreeView => match action {
                Action::Quit => {
                    state.should_quit = true;
                }
                Action::Reload => {
                    if let Err(e) = app.reload().await {
                        error!("Failed to reload application state: {}", e);
                    }
                }
                Action::SwitchFocus => match state.widget_focus {
                    WidgetFocus::TreeView => state.widget_focus = WidgetFocus::Details,
                    WidgetFocus::Details => state.widget_focus = WidgetFocus::TreeView,
                },
                Action::Up => {
                    app.state.tree_state.key_up();
                    app.state.selected_node_id = app.state.tree_state.selected().last().cloned();
                }
                Action::Down => {
                    app.state.tree_state.key_down();
                    app.state.selected_node_id = app.state.tree_state.selected().last().cloned();
                }
                Action::OpenNode => {
                    let identifiers = app.state.tree_state.selected().to_vec();
                    app.state.tree_state.open(identifiers);
                    app.state.selected_node_id = app.state.tree_state.selected().last().cloned();
                }
                Action::CloseNode => {
                    let identifiers = app.state.tree_state.selected().to_vec();
                    app.state.tree_state.close(&identifiers);
                    app.state.selected_node_id = app.state.tree_state.selected().last().cloned();
                }
            },
            WidgetFocus::Details => match action {
                Action::Quit => {
                    state.should_quit = true;
                }
                Action::Reload => {
                    if let Err(e) = app.reload().await {
                        error!("Failed to reload application state: {}", e);
                    }
                }
                Action::SwitchFocus => {}
                Action::Up => {}
                Action::Down => {}
                Action::OpenNode => {}
                Action::CloseNode => {}
            },
        }
    }
}
