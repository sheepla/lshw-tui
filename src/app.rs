use tui_tree_widget::{TreeItem, TreeState};

use crate::core::lshw::run_lshw;
use crate::core::types::HardwareNode;
use crate::debug::get_dummy_data; // Only get_dummy_data remains
use crate::state::{AppNode, NodeId, NodeMap, State};
use tracing::{error, info}; // Use tracing macros

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Failed to execute lshw: {0}")]
    Lshw(std::io::Error),
}

#[derive(Debug)]
pub struct App {
    pub state: State,
    pub nodes: NodeMap,
    pub root_id: NodeId,
    pub tree_items: Vec<TreeItem<'static, NodeId>>,
    pub lshw_command: String,
    pub sanitize: bool,
}

impl App {
    pub async fn init(lshw_command: &str, sanitize: bool) -> Result<Self, AppError> {
        let lshw_result = run_lshw(lshw_command, sanitize).await;

        let (hw_root_node_dto, lshw_error) = match lshw_result {
            Ok(node) => (node, None),
            Err(e) => (get_dummy_data(), Some(e)),
        };

        if let Some(e) = lshw_error.as_ref() {
            error!("lshw command failed: {:?}", e);
        }
        info!("--- Hardware Node Data ---\n{:#?}", hw_root_node_dto);

        let mut nodes = NodeMap::new();
        let mut next_id = 0;
        let root_id =
            Self::build_node_map_and_root_id(&hw_root_node_dto, None, &mut next_id, &mut nodes);

        let tree_items = vec![Self::build_tree_items_from_map(root_id, &nodes)];
        let mut tree_state = TreeState::default();
        tree_state.select_first(); // Ensure the first item is selected
        let mut state = State::new(tree_state);

        // Set the initial selected_node_id in the state
        if let Some(first_item_id) = tree_items.first().map(|item| item.identifier().clone()) {
            state.selected_node_id = Some(first_item_id);
        }

        Ok(Self {
            state,
            nodes,
            root_id,
            tree_items,
            lshw_command: lshw_command.to_string(),
            sanitize,
        })
    }

    pub async fn reload(&mut self) -> Result<(), AppError> {
        let lshw_result = run_lshw(&self.lshw_command, self.sanitize).await;

        let (hw_root_node_dto, lshw_error) = match lshw_result {
            Ok(node) => (node, None),
            Err(e) => (get_dummy_data(), Some(e)),
        };

        if let Some(e) = lshw_error.as_ref() {
            error!("lshw command failed: {:?}", e);
        }
        info!("--- Hardware Node Data ---\n{:#?}", hw_root_node_dto);

        let mut nodes = NodeMap::new();
        let mut next_id = 0;
        let root_id =
            Self::build_node_map_and_root_id(&hw_root_node_dto, None, &mut next_id, &mut nodes);

        let tree_items = vec![Self::build_tree_items_from_map(root_id, &nodes)];
        let mut tree_state = TreeState::default();
        tree_state.select_first(); // Ensure the first item is selected

        self.nodes = nodes;
        self.root_id = root_id;
        self.tree_items = tree_items;
        self.state.tree_state = tree_state;

        // Set the initial selected_node_id in the state
        if let Some(first_item_id) = self.tree_items.first().map(|item| item.identifier().clone()) {
            self.state.selected_node_id = Some(first_item_id);
        } else {
            self.state.selected_node_id = None;
        }

        Ok(())
    }

    pub fn get_selected_node(&self) -> Option<&HardwareNode> {
        self.state
            .selected_node_id
            .and_then(|id| self.nodes.get(&id))
            .map(|app_node| &app_node.data)
    }

    // Helper to recursively build AppNodes and populate the NodeMap
    fn build_node_map_and_root_id(
        hw_node: &HardwareNode,
        parent_id: Option<NodeId>,
        next_id: &mut NodeId,
        nodes: &mut NodeMap,
    ) -> NodeId {
        let current_id = *next_id;
        *next_id += 1;

        let mut children_ids = Vec::new();
        if let Some(children_hw_nodes) = &hw_node.children {
            for child_hw_node in children_hw_nodes {
                let child_id = Self::build_node_map_and_root_id(
                    child_hw_node,
                    Some(current_id),
                    next_id,
                    nodes,
                );
                children_ids.push(child_id);
            }
        }

        nodes.insert(
            current_id,
            AppNode {
                id: current_id,
                parent_id,
                children_ids,
                data: hw_node.clone(),
            },
        );
        current_id
    }

    // Helper to build TreeItems from the NodeMap
    fn build_tree_items_from_map(node_id: NodeId, nodes: &NodeMap) -> TreeItem<'static, NodeId> {
        let app_node = nodes.get(&node_id).expect("Node not found in map");
        let id = &app_node.data.id;
        let name = app_node
            .data
            .description
            .as_ref()
            .and_then(|v| v.as_str())
            .map(|s| format!("[{}] {}", id, s))
            .unwrap_or_else(|| id.clone());

        let children_tree_items: Vec<TreeItem<'static, NodeId>> = app_node
            .children_ids
            .iter()
            .map(|&child_id| Self::build_tree_items_from_map(child_id, nodes))
            .collect();

        if children_tree_items.is_empty() {
            TreeItem::new_leaf(node_id, name)
        } else {
            TreeItem::new(node_id, name, children_tree_items).expect("Failed to create tree item")
        }
    }

    pub fn tick(&self) {}
}
