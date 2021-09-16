use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(remote = "RectOffset")]
pub struct RectOffsetDef {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}