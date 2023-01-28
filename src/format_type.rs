use strum_macros::Display;

#[derive(Display, Debug, Clone, Copy)]
pub enum BlockDisplayFormatType {
    #[strum(serialize = "opensea")]
    OPENSEA,
}

impl From<&str> for BlockDisplayFormatType {
    fn from(format: &str) -> Self {
        match format {
            "OPENSEA" => BlockDisplayFormatType::OPENSEA,
            _ => BlockDisplayFormatType::OPENSEA,
        }
    }
}
