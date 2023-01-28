pub struct OpenseaMetadataV1AttrJson {
    trait_type: Option<String>,
    value: Option<String>,
}

impl From<Block> for OpenseaMetadataAttrJson {
    fn from(block: Block) -> OpenseaMetadataAttrJson {
        Self {
            trait_type: "",
            value: "",
        }
    }
}
