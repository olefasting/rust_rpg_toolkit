use crate::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoiseLevel {
    None,
    Silent,
    Moderate,
    Loud,
    Extreme,
}

impl NoiseLevel {
    const RADIUS_NONE: f32 = 0.0;
    const RADIUS_SILENT: f32 = 64.0;
    const RADIUS_MODERATE: f32 = 360.0;
    const RADIUS_LOUD: f32 = 720.0;
    const RADIUS_EXTREME: f32 = 1440.0;

    pub fn to_range(self) -> f32 {
        match self {
            Self::None => Self::RADIUS_NONE,
            Self::Silent => Self::RADIUS_SILENT,
            Self::Moderate => Self::RADIUS_MODERATE,
            Self::Loud => Self::RADIUS_LOUD,
            Self::Extreme => Self::RADIUS_EXTREME,
        }
    }
}

impl Default for NoiseLevel {
    fn default() -> Self {
        Self::None
    }
}

impl ToString for NoiseLevel {
    fn to_string(&self) -> String {
        let res = match self {
            Self::None => "None",
            Self::Silent => "Silent",
            Self::Moderate => "Moderate",
            Self::Loud => "Loud",
            Self::Extreme => "Extreme",
        };
        res.to_string()
    }
}
