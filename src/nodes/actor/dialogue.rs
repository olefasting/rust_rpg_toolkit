use macroquad::{
    experimental::{
        collections::storage,
    },
};

use serde::{
    Serialize,
    Deserialize,
};

use crate::Resources;

use super::Actor;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ActorDialogueRequirement {
    #[serde(rename = "active_mission")]
    ActiveMission { mission_id: String },
    #[serde(rename = "is_in_faction")]
    IsInFaction { faction_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ActorDialogueAction {
    #[serde(rename = "open_trade")]
    OpenTrade,
    #[serde(rename = "complete_mission")]
    CompleteMission { mission_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorDialogue {
    pub id: String,
    #[serde(default)]
    pub title: String,
    pub body: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub requirements: Vec<ActorDialogueRequirement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ActorDialogueAction>,
    #[serde(skip)]
    pub actor_name: String,
}

impl ActorDialogue {
    pub fn get_options(&self, actor: &Actor) -> Vec<Self> {
        let resources = storage::get::<Resources>();
        let mut dialogue = Vec::new();
        'option: for option_id in &self.options {
            let option = resources.dialogue.get(option_id).unwrap();
            for requirement in &option.requirements {
                match requirement {
                    ActorDialogueRequirement::ActiveMission { mission_id } => {
                        if actor.active_missions.iter().find(|mission| mission.id == mission_id.clone()).is_none() {
                            break 'option;
                        }
                    },
                    ActorDialogueRequirement::IsInFaction { faction_id } => {
                        if actor.factions.contains(&faction_id) == false {
                            break 'option;
                        }
                    }
                }
                let mut option = option.clone();
                option.actor_name = self.actor_name.clone();
                dialogue.push(option);
            }
        }
        dialogue
    }

    pub fn apply_action(&self, actor: &mut Actor) {
        if let Some(action) = self.action.clone() {
            match action {
                ActorDialogueAction::CompleteMission { mission_id } => {
                    let mut active_missions = actor.active_missions.clone();
                    active_missions.retain(|mission| {
                        if mission.id == mission_id {
                            actor.completed_missions.push(mission.clone());
                            return false;
                        }
                        true
                    });
                    actor.active_missions = active_missions;
                },
                _ => {}
            }
        }
    }
}

impl Default for ActorDialogue {
    fn default() -> Self {
        ActorDialogue {
            id: "".to_string(),
            actor_name: "".to_string(),
            title: "...".to_string(),
            body: Vec::new(),
            response: None,
            options: Vec::new(),
            requirements: Vec::new(),
            action: None,
        }
    }
}
