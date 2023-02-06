use marine_rs_sdk::marine;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
#[marine]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Block {
    pub timestamp: u64,
    pub name: String,
    pub content: String,
    pub previous: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockContent {
  pub name: String,
  pub data: Value,
}
impl Block {
    pub fn to_value(block: Block) -> (String, Value) {
        let content: BlockContent = serde_json::from_str(&block.content).unwrap();
       
        (content.name, content.data)
    }
}
