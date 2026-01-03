use crate::core::types::HardwareNode;
use std::collections::HashMap;
use tui_tree_widget::TreeState;

pub type NodeId = usize;

#[derive(Debug, Clone)]
pub struct AppNode {
    pub id: NodeId,
    pub parent_id: Option<NodeId>,
    pub children_ids: Vec<NodeId>,
    pub data: HardwareNode,
}

pub type NodeMap = HashMap<NodeId, AppNode>;

#[derive(Debug, Default)]
pub struct State {
    pub should_quit: bool,
    pub widget_focus: WidgetFocus,
    pub tree_state: TreeState<NodeId>,
    pub selected_node_id: Option<NodeId>,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum WidgetFocus {
    #[default]
    TreeView,
    Details,
}

impl State {
    pub fn new(mut tree_state: TreeState<NodeId>) -> Self {
        tree_state.select_first();
        Self {
            should_quit: false,
            widget_focus: WidgetFocus::default(),
            tree_state,
            selected_node_id: None,
        }
    }
}
