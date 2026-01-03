use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct HardwareNode {
    pub id: String,
    pub class: String,
    pub description: Option<Value>,
    pub vendor: Option<Value>,
    pub product: Option<Value>,
    pub serial: Option<Value>,
    pub slot: Option<Value>,
    pub businfo: Option<Value>,
    pub version: Option<Value>,
    pub width: Option<Value>,
    pub clock: Option<Value>,
    pub physid: Option<Value>,
    pub logicalname: Option<Value>,
    pub dev: Option<Value>,
    pub date: Option<Value>,
    pub handle: Option<Value>,
    pub size: Option<Value>,
    pub capacity: Option<Value>,
    pub disabled: Option<Value>,
    pub claimed: Option<Value>,
    pub units: Option<Value>,
    pub capabilities: Option<Value>,
    pub configuration: Option<Value>,
    pub children: Option<Vec<HardwareNode>>,
}