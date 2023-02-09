use marine_rs_sdk::marine;
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
#[marine]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Block {
    pub timestamp: u64,
    pub name: String,
    pub content: String,
    pub previous: String,
}

impl Block {
    pub fn to_value(block: Block) -> (String, Value) {
        let content_json = match to_value(block.content.clone()) {
            Ok(data) => data,
            Err(_) => serde_json::from_str(&block.content).unwrap_or_default(),
        };

        (block.name, content_json)
    }
}
