use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DialogueRequirement {
    ActiveMission { mission_id: String },
    CompletedMission { mission_id: String },
    IsInFaction { faction_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DialogueAction {
    OpenTrade,
    StartMission { mission_id: String },
    CompleteMission { mission_id: String },
    MapTransition { chapter_index: usize, map_id: String },
    CompleteChapter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dialogue {
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub body: Vec<String>,
    #[serde(default)]
    pub response: Vec<String>,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub requirements: Vec<DialogueRequirement>,
    #[serde(default)]
    pub exclusions: Vec<DialogueRequirement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<DialogueAction>,
    #[serde(skip)]
    pub actor_name: String,
    #[serde(skip)]
    pub should_apply: bool,
}

impl Dialogue {
    pub fn get_options(&self, actor: &Actor) -> Vec<Self> {
        let resources = storage::get::<Resources>();
        let mut dialogue = Vec::new();
        'option: for option_id in &self.options {
            let option = resources.dialogue.get(option_id).unwrap();
            for requirement in &option.requirements {
                match requirement {
                    DialogueRequirement::ActiveMission { mission_id } => {
                        if actor.active_missions.iter().find(|mission| mission.id == mission_id.clone()).is_none() {
                            continue 'option;
                        }
                    },
                    DialogueRequirement::CompletedMission { mission_id } => {
                        if actor.completed_missions.iter().find(|mission| mission.id == mission_id.clone()).is_none() {
                            continue 'option;
                        }
                    }
                    DialogueRequirement::IsInFaction { faction_id } => {
                        if actor.factions.contains(&faction_id) == false {
                            continue 'option;
                        }
                    }
                }
            }
            for exclusion in &option.exclusions {
                match exclusion {
                    DialogueRequirement::ActiveMission { mission_id } => {
                        if actor.active_missions.iter().find(|mission| mission.id == mission_id.clone()).is_some() {
                            continue 'option;
                        }
                    },
                    DialogueRequirement::CompletedMission { mission_id } => {
                        if actor.completed_missions.iter().find(|mission| mission.id == mission_id.clone()).is_some() {
                            continue 'option;
                        }
                    }
                    DialogueRequirement::IsInFaction { faction_id } => {
                        if actor.factions.contains(&faction_id) {
                            continue 'option;
                        }
                    }
                }
            }
            let mut option = option.clone();
            option.actor_name = self.actor_name.clone();
            dialogue.push(option);
        }
        dialogue
    }

    pub fn apply_action(&self, actor: &mut Actor) {
        if let Some(action) = self.action.clone() {
            let resources = storage::get::<Resources>();
            match action {
                DialogueAction::OpenTrade => { todo!() },
                DialogueAction::CompleteMission { mission_id } => {
                    actor.active_missions = actor.active_missions
                        .clone()
                        .into_iter()
                        .map(|mut mission| {
                        if mission.id == mission_id {
                            mission.objectives = mission.objectives
                                .into_iter()
                                .map(|(objective, _)| (objective, true))
                                .collect();
                        }
                        mission.is_completed = true;
                        mission
                    }).collect();
                },
                DialogueAction::StartMission { mission_id } => {
                    let params = resources.missions.get(&mission_id).cloned().unwrap();
                    actor.active_missions.push(Mission::new(params));
                },
                DialogueAction::MapTransition { chapter_index, map_id } => {
                    actor.current_dialogue = None;
                    dispatch_event(Event::ChangeMap(chapter_index, map_id));
                },
                DialogueAction::CompleteChapter => todo!(),
            }
        }
    }
}

impl Default for Dialogue {
    fn default() -> Self {
        Dialogue {
            id: "".to_string(),
            actor_name: "".to_string(),
            title: "...".to_string(),
            body: Vec::new(),
            response: Vec::new(),
            options: Vec::new(),
            requirements: Vec::new(),
            exclusions: Vec::new(),
            action: None,
            should_apply: false,
        }
    }
}
