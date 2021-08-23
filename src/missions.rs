use macroquad::prelude::*;

use serde::{
    Serialize,
    Deserialize,
};

use crate::nodes::{Actor, GameState};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MissionObjective {
    #[serde(rename = "kill")]
    Kill { instance_id: String },
    #[serde(rename = "retrieve_item")]
    RetrieveItem { instance_id: String },
    #[serde(rename = "deliver_item")]
    DeliverItem { instance_id: String },
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
    pub description: String,
    pub objectives: Vec<MissionObjective>,
    pub rewards: Vec<MissionReward>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_mission_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Mission {
    pub id: String,
    pub title: String,
    pub description: String,
    pub objectives: Vec<(MissionObjective, bool)>,
    pub rewards: Vec<MissionReward>,
    pub next_mission_id: Option<String>,
    pub is_completed: bool,
}

impl Mission {
    pub fn new(params: MissionParams) -> Self {
        Mission {
            id: params.id,
            title: params.title,
            description: params.description,
            objectives: params.objectives.into_iter().map(|objective| (objective, false)).collect(),
            rewards: params.rewards,
            next_mission_id: params.next_mission_id,
            is_completed: false,
        }
    }
}
