mod block;
mod format_type;

use block::Block;
use format_type::BlockDisplayFormatType;
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

#[marine]
pub fn serialize(name: String, content: String, previous_cid: String) -> String {
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let milliseconds = timestamp.as_millis();

    let data = Block {
        timestamp: milliseconds as u64,
        name,
        content,
        previous: previous_cid,
    };

    serde_json::to_string(&data).unwrap_or("".to_string())
}

#[marine]
pub fn deserialize(json: &String) -> Block {
    serde_json::from_str(json).unwrap()
}

#[marine]
pub fn format(format_type: String, blocks: Vec<Block>) -> String {
    let format_type = format_type.as_str().into();

    let block_map: BTreeMap<String, Value> = match format_type {
        BlockDisplayFormatType::OPENSEA => blocks
            .into_iter()
            .map(|block| Block::to_value(block))
            .collect(),
    };

    json!(block_map).to_string()
}
