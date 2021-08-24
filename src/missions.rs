use macroquad::prelude::*;

use serde::{
    Serialize,
    Deserialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MissionObjective {
    #[serde(rename = "kill")]
    Kill { instance_id: String },
    #[serde(rename = "find_item")]
    FindItem { prototype_id: String },
    #[serde(rename = "deliver_item")]
    DeliverItem { prototype_id: String },
    #[serde(rename = "go_to_location")]
    GoToWaypoint { waypoint_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MissionReward {
    #[serde(rename = "item")]
    Item { prototype_id: String, amount: u32 },
    #[serde(rename = "credits")]
    Credits { amount: u32 },
    #[serde(rename = "xp", alias = "experience")]
    Experience { amount: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionParams {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub objectives: Vec<MissionObjective>,
    #[serde(default)]
    pub rewards: Vec<MissionReward>,
    #[serde(default, rename = "next_missions")]
    pub next_mission_ids: Vec<String>,
    #[serde(default)]
    pub no_autocompletion: bool,
}

impl Default for MissionParams {
    fn default() -> Self {
        MissionParams {
            id: "".to_string(),
            title: "".to_string(),
            description: "".to_string(),
            objectives: Vec::new(),
            rewards: Vec::new(),
            next_mission_ids: Vec::new(),
            no_autocompletion: false
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mission {
    pub id: String,
    pub title: String,
    pub description: String,
    pub objectives: Vec<(MissionObjective, bool)>,
    pub rewards: Vec<MissionReward>,
    pub next_mission_ids: Vec<String>,
    pub is_completed: bool,
    pub no_autocompletion: bool,
}

impl Mission {
    pub fn new(params: MissionParams) -> Self {
        Mission {
            id: params.id,
            title: params.title,
            description: params.description,
            objectives: params.objectives.into_iter().map(|objective| (objective, false)).collect(),
            rewards: params.rewards,
            next_mission_ids: params.next_mission_ids,
            is_completed: false,
            no_autocompletion: params.no_autocompletion,
        }
    }
}
