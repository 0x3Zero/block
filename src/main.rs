mod block;

use block::Block;
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;
use std::time::{SystemTime, UNIX_EPOCH};

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

#[marine]
pub fn serialize(content: String, previous_cid: String) -> String {
    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let milliseconds = timestamp.as_millis();

    let data = Block {
        timestamp: milliseconds as u64,
        content,
        previous: previous_cid,
    };

    serde_json::to_string(&data).unwrap_or("".to_string())
}

#[marine]
pub fn deserialize(json: &String) -> Block {
    serde_json::from_str(json).unwrap()
}
