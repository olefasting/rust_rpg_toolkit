use macroquad::{color, prelude::*};

use serde::{Deserialize, Serialize};

use crate::{
    json,
    nodes::{Actor, Item},
};

const MISSION_MARKER_Y_OFFSET: f32 = 16.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MissionObjective {
    Kill { actor_id: String },
    FindItem { item_id: String },
    DeliverItem { item_id: String },
    GoToWaypoint { waypoint_id: String },
}

impl MissionObjective {
    pub fn get_marker_color(&self) -> Option<Color> {
        match self.clone() {
            Self::Kill { actor_id: _ } => Some(color::RED),
            Self::FindItem { item_id: _ } => Some(color::YELLOW),
            _ => None,
        }
    }

    pub fn get_marker_position(&self) -> Option<Vec2> {
        match self.clone() {
            Self::Kill { actor_id } => {
                for actor in scene::find_nodes_by_type::<Actor>() {
                    if actor.id == actor_id {
                        return Some(vec2(
                            actor.body.position.x,
                            actor.body.position.y - MISSION_MARKER_Y_OFFSET,
                        ));
                    }
                }
                None
            }
            Self::FindItem { item_id } => {
                for item in scene::find_nodes_by_type::<Item>() {
                    if item.id == item_id {
                        return Some(vec2(
                            item.position.x,
                            item.position.y - MISSION_MARKER_Y_OFFSET,
                        ));
                    }
                }
                None
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MissionReward {
    Item {
        prototype_id: String,
        amount: u32,
    },
    Credits {
        amount: u32,
    },
    #[serde(rename = "xp", alias = "experience")]
    Experience {
        amount: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MissionMarker {
    Actor {
        actor_id: String,
    },
    Item {
        item_id: String,
    },
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
                        return Some(vec2(
                            actor.body.position.x,
                            actor.body.position.y - MISSION_MARKER_Y_OFFSET,
                        ));
                    }
                }
                None
            }
            Self::Item { item_id } => {
                for item in scene::find_nodes_by_type::<Item>() {
                    if item.id == item_id {
                        return Some(vec2(
                            item.position.x,
                            item.position.y - MISSION_MARKER_Y_OFFSET,
                        ));
                    }
                }
                None
            }
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
            no_autocompletion: false,
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
            objectives: params
                .objectives
                .into_iter()
                .map(|objective| (objective, false))
                .collect(),
            rewards: params.rewards,
            next_mission_ids: params.next_mission_ids,
            marker: params.marker,
            is_completed: false,
            no_autocompletion: params.no_autocompletion,
        }
    }
}
