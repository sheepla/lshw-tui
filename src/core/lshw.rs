use crate::core::types::HardwareNode;
use tracing::error; // Use tracing macros
// Removed: use std::process::Command;

pub async fn run_lshw(lshw_command: &str, sanitize: bool) -> std::io::Result<HardwareNode> {
    let mut cmd = std::process::Command::new(lshw_command);

    cmd.arg("-json");
    if sanitize {
        cmd.arg("-sanitize");
    }

    let output = cmd.output()?;
    let json_str = String::from_utf8(output.stdout)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    match serde_json::from_str(&json_str) {
        Ok(node) => Ok(node),
        Err(e_serde) => {
            error!(
                "Failed to deserialize lshw JSON. Error: {}. Raw JSON:
{}",
                e_serde, json_str
            );
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to parse lshw JSON: {}", e_serde),
            ))
        }
    }
}
