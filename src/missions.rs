use macroquad::{
    color,
    prelude::*
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::{
    nodes::{Actor, Item},
    json,
};

const MISSION_MARKER_Y_OFFSET: f32 = 16.0;

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

impl MissionObjective {
    pub fn get_marker_color(&self) -> Option<Color> {
        match self.clone() {
            Self::Kill { instance_id: _ } => Some(color::RED),
            Self::FindItem { prototype_id: _ } => Some(color::YELLOW),
            _ => None,
        }
    }

    pub fn get_marker_position(&self) -> Option<Vec2> {
        match self.clone() {
            Self::Kill { instance_id } => {
                for actor in scene::find_nodes_by_type::<Actor>() {
                    if actor.id == instance_id {
                        return Some(vec2(actor.body.position.x, actor.body.position.y - MISSION_MARKER_Y_OFFSET));
                    }
                }
                None
            },
            Self::FindItem { prototype_id } => {
                for item in scene::find_nodes_by_type::<Item>() {
                    if item.prototype_id == prototype_id {
                        return Some(vec2(item.position.x, item.position.y - MISSION_MARKER_Y_OFFSET));
                    }
                }
                None
            },
            _ => None,
        }
    }
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
#[serde(tag = "type")]
pub enum MissionMarker {
    #[serde(rename = "actor")]
    Actor { actor_id: String },
    #[serde(rename = "item")]
    Item { item_id: String },
    #[serde(rename = "location")]
    Location {
        #[serde(with = "json::def_vec2")]
        target: Vec2,
    },
}

impl MissionMarker {
    pub fn get_color(&self) -> Color {
        match self.clone() {
            Self::Actor { actor_id: _ } => color::YELLOW,
            Self::Item { item_id: _ } => color::YELLOW,
            Self::Location { target: _ } => color::YELLOW,
        }
    }

    pub fn get_position(&self) -> Option<Vec2> {
        match self.clone() {
            Self::Actor { actor_id } => {
                for actor in scene::find_nodes_by_type::<Actor>() {
                    if actor.id == actor_id {
                        return Some(vec2(actor.body.position.x, actor.body.position.y - MISSION_MARKER_Y_OFFSET));
                    }
                }
                None
            },
            Self::Item { item_id } => {
                for item in scene::find_nodes_by_type::<Item>() {
                    if item.id == item_id {
                        return Some(vec2(item.position.x, item.position.y - MISSION_MARKER_Y_OFFSET));
                    }
                }
                None
            },
            Self::Location { target } => Some(target),
        }
    }
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<MissionMarker>,
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
            marker: None,
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
    pub marker: Option<MissionMarker>,
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
            marker: params.marker,
            is_completed: false,
            no_autocompletion: params.no_autocompletion,
        }
    }
}
