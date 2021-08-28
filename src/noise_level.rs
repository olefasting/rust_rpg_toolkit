use std::cmp::Ordering;

use crate::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum NoiseLevel {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "silent")]
    Silent,
    #[serde(rename = "moderate")]
    Moderate,
    #[serde(rename = "loud")]
    Loud,
    #[serde(rename = "extreme")]
    Extreme,
}

impl NoiseLevel {
    const RADIUS_NONE: f32 = 0.0;
    const RADIUS_SILENT: f32 = 64.0;
    const RADIUS_MODERATE: f32 = 192.0;
    const RADIUS_LOUD: f32 = 416.0;
    const RADIUS_EXTREME: f32 = 1024.0;

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

impl Ord for NoiseLevel {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::None => match other {
                Self::None => Ordering::Equal,
                _ => Ordering::Less,
            },
            Self::Silent => match other {
                Self::None => Ordering::Greater,
                Self::Silent => Ordering::Equal,
                _ => Ordering::Less,
            },
            Self::Moderate => match other {
                Self::None | Self::Silent => Ordering::Greater,
                Self::Moderate => Ordering::Equal,
                _ => Ordering::Less,
            },
            Self::Loud => match other {
                Self::None | Self::Silent | Self::Moderate => Ordering::Greater,
                Self::Loud => Ordering::Equal,
                _ => Ordering::Less,
            },
            Self::Extreme => match other {
                Self::None | Self::Silent | Self::Moderate | Self::Loud => Ordering::Greater,
                Self::Extreme => Ordering::Equal,
            }
        }
    }
}
