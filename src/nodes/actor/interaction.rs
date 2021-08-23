use serde::{
    Serialize,
    Deserialize,
};

use super::Actor;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ActorInteractionRequirement {
    #[serde(rename = "active_mission")]
    ActiveMission { mission_id: String },
    #[serde(rename = "is_in_faction")]
    IsInFaction { faction_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ActorInteractionAction {
    #[serde(rename = "open_trade")]
    OpenTrade,
    #[serde(rename = "complete_mission")]
    CompleteMission { mission_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorInteraction {
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub response_options: Vec<ActorInteraction>,
    #[serde(default)]
    pub requirements: Vec<ActorInteractionRequirement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ActorInteractionAction>,
}

impl ActorInteraction {
    pub fn get_options(&self, actor: &Actor) -> Vec<Self> {
        let mut interactions = Vec::new();
        'interaction: for interaction in &self.response_options {
            for requirement in &interaction.requirements {
                match requirement {
                    ActorInteractionRequirement::ActiveMission { mission_id } => {
                        if actor.active_missions.iter().find(|mission| mission.id == mission_id.clone()).is_none() {
                            break 'interaction;
                        }
                    },
                    ActorInteractionRequirement::IsInFaction { faction_id } => {
                        if actor.factions.contains(faction_id) == false {
                            break 'interaction;
                        }
                    }
                }
                interactions.push(interaction.clone());
            }
        }
        interactions
    }

    pub fn apply_action(&self, actor: &mut Actor) {
        if let Some(action) = self.action.clone() {
            match action {
                ActorInteractionAction::CompleteMission { mission_id } => {
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

impl Default for ActorInteraction {
    fn default() -> Self {
        ActorInteraction {
            title: "...".to_string(),
            body: "...".to_string(),
            response_options: Vec::new(),
            requirements: Vec::new(),
            action: None,
        }
    }
}
