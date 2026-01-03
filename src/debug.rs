use crate::core::types::HardwareNode;
use serde_json::Value; // Add this import

pub fn get_dummy_data() -> HardwareNode {
    HardwareNode {
        id: "core".to_string(),
        class: "system".to_string(),
        description: Some(Value::String("My Computer (Dummy Data)".to_string())),
        physid: Some(Value::String("core".to_string())), // Keep physid for dummy data to match original logic
        children: Some(vec![
            HardwareNode {
                id: "memory".to_string(),
                class: "memory".to_string(),
                description: Some(Value::String("System Memory".to_string())),
                size: Some(Value::Number(8192.into())),
                physid: Some(Value::String("memory".to_string())), // Keep physid for dummy data
                children: None, // Explicitly None for leaf node
                ..HardwareNode::default()
            },
            HardwareNode {
                id: "cpu".to_string(),
                class: "processor".to_string(),
                description: Some(Value::String("CPU".to_string())),
                product: Some(Value::String("Awesome CPU".to_string())),
                physid: Some(Value::String("cpu".to_string())), // Keep physid for dummy data
                children: Some(vec![HardwareNode {
                    id: "cache:0".to_string(),
                    class: "cache".to_string(),
                    description: Some(Value::String("L1 Cache".to_string())),
                    physid: Some(Value::String("cache:0".to_string())), // Keep physid for dummy data
                    children: None, // Explicitly None for leaf node
                    ..HardwareNode::default()
                }]),
                ..HardwareNode::default()
            },
        ]),
        ..HardwareNode::default()
    }
}