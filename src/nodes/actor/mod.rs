use crate::prelude::*;

use mode::Family;

pub use behavior::{
    ActorAggression,
    ActorBehavior,
    ActorBehaviorParams,
    ActorBehaviorFamily,
    TestMode,
};
pub use controller::{
    ActorController,
    ActorControllerKind,
};

pub use stats::ActorStats;

mod controller;
mod stats;
mod behavior;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorParams {
    pub id: String,
    pub name: String,
    pub factions: Vec<String>,
    #[serde(default)]
    pub behavior: ActorBehaviorParams,
    #[serde(default, with = "json::opt_vec2", skip_serializing_if = "Option::is_none")]
    pub position: Option<Vec2>,
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub intelligence: u32,
    pub willpower: u32,
    pub perception: u32,
    pub charisma: u32,
    #[serde(default)]
    pub current_health: f32,
    #[serde(default)]
    pub current_stamina: f32,
    #[serde(default)]
    pub current_energy: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collider: Option<Collider>,
    #[serde(default, flatten)]
    pub inventory: InventoryParams,
    #[serde(default)]
    pub equipped_items: EquippedItems,
    pub animation_player: SpriteAnimationParams,
    #[serde(default)]
    pub experience: u32,
    #[serde(default)]
    pub can_level_up: bool,
    #[serde(default, rename = "dialogue", skip_serializing_if = "Option::is_none")]
    pub dialogue_id: Option<String>,
}

impl Default for ActorParams {
    fn default() -> Self {
        ActorParams {
            id: generate_id(),
            name: "Unnamed Actor".to_string(),
            factions: Vec::new(),
            behavior: Default::default(),
            position: None,
            strength: 8,
            dexterity: 8,
            constitution: 8,
            intelligence: 8,
            willpower: 8,
            perception: 8,
            charisma: 8,
            current_health: 0.0,
            current_stamina: 0.0,
            current_energy: 0.0,
            collider: None,
            inventory: Default::default(),
            equipped_items: Default::default(),
            animation_player: Default::default(),
            experience: 0,
            can_level_up: false,
            dialogue_id: None,
        }
    }
}

impl Into<ActorStats> for ActorParams {
    fn into(self) -> ActorStats {
        ActorStats {
            strength: self.strength,
            dexterity: self.dexterity,
            constitution: self.constitution,
            intelligence: self.intelligence,
            willpower: self.willpower,
            perception: self.perception,
            charisma: self.charisma,
            current_health: self.current_health,
            current_stamina: self.current_stamina,
            current_energy: self.current_energy,
            ..Default::default()
        }
    }
}

impl Into<SavedCharacter> for ActorParams {
    fn into(self) -> SavedCharacter {
        let game_params = storage::get::<GameParams>();
        let resources = storage::get::<Resources>();
        let mut item_ids = Vec::new();
        let mut items = Vec::new();

        for entry in self.inventory.items {
            let id = generate_id();
            let params = resources.items.get(&entry).cloned().unwrap();
            items.push(ItemParams {
                id: id.clone(),
                ..params
            });
            item_ids.push(id);
        }

        let current_chapter_index = 0;
        let current_map_id = resources.chapters
            .get(current_chapter_index)
            .unwrap()
            .initial_map_id
            .clone();

        SavedCharacter {
            game_version: game_params.game_version.clone(),
            actor: ActorParams {
                id: generate_id(),
                inventory: InventoryParams {
                    items: item_ids,
                    credits: self.inventory.credits,
                },
                ..self
            },
            items,
            active_missions: Vec::new(),
            completed_missions: Vec::new(),
            current_chapter_index: 0,
            current_map_id,
        }
    }
}

pub struct Actor {
    pub id: String,
    pub name: String,
    pub active_missions: Vec<Mission>,
    pub completed_missions: Vec<Mission>,
    pub noise_level: NoiseLevel,
    pub behavior: ActorBehaviorParams,
    pub stats: ActorStats,
    pub factions: Vec<String>,
    pub body: PhysicsBody,
    pub inventory: Inventory,
    pub equipped_items: EquippedItems,
    pub primary_ability: Option<Ability>,
    pub secondary_ability: Option<Ability>,
    pub controller: ActorController,
    pub experience: u32,
    pub dialogue: Option<Dialogue>,
    pub current_dialogue: Option<Dialogue>,
    pub game_state: Handle<GameState>,
    animation_player: SpriteAnimationPlayer,
    automaton: Automaton<ActorBehaviorFamily>,
    noise_level_timer: f32,
    can_level_up: bool,
}

impl Actor {
    pub const HEALTH_BAR_LENGTH: f32 = 50.0;
    pub const HEALTH_BAR_HEIGHT: f32 = 10.0;
    pub const HEALTH_BAR_OFFSET_Y: f32 = 25.0;

    const ENCUMBERED_SPEED_FACTOR: f32 = 0.1;

    const SPRINT_SPEED_FACTOR: f32 = 2.0;
    const SPRINT_STAMINA_COST: f32 = 10.0;

    const MOVE_NOISE_LEVEL: NoiseLevel = NoiseLevel::Silent;
    const SPRINT_NOISE_LEVEL: NoiseLevel = NoiseLevel::Moderate;

    const NOISE_LEVEL_COOLDOWN: f32 = 1.5;

    const PICK_UP_RADIUS: f32 = 36.0;
    const INTERACT_RADIUS: f32 = 36.0;

    pub fn new(game_state: Handle<GameState>, controller_kind: ActorControllerKind, params: ActorParams) -> Self {
        let position = params.position.unwrap_or_default();
        let dialogue = if let Some(dialogue_id) = params.dialogue_id.clone() {
            let resources = storage::get::<Resources>();
            let mut dialogue = resources.dialogue.get(&dialogue_id).cloned().unwrap();
            dialogue.actor_name = params.name.clone();
            Some(dialogue)
        } else {
            None
        };

        let inventory = Inventory::from_prototypes(&params.inventory);

        let mode = Box::new(TestMode {});

        Actor {
            id: params.id.clone(),
            name: params.name.clone(),
            active_missions: Vec::new(),
            completed_missions: Vec::new(),
            noise_level: NoiseLevel::None,
            behavior: params.behavior.clone(),
            stats: params.clone().into(),
            factions: params.factions,
            body: PhysicsBody::new(position, 0.0, params.collider),
            inventory,
            equipped_items: params.equipped_items,
            primary_ability: None,
            secondary_ability: None,
            controller: ActorController::new(controller_kind),
            experience: params.experience,
            dialogue,
            current_dialogue: None,
            animation_player: SpriteAnimationPlayer::new(params.animation_player.clone()),
            noise_level_timer: 0.0,
            can_level_up: params.can_level_up,
            automaton: ActorBehaviorFamily::automaton_with_mode(mode),
            game_state,
        }
    }

    pub fn add_node(game_state: Handle<GameState>, controller_kind: ActorControllerKind, params: ActorParams) -> Handle<Self> {
        scene::add_node(Self::new(game_state, controller_kind, params))
    }

    pub fn to_params(&self) -> ActorParams {
        let dialogue_id = if let Some(dialogue) = &self.dialogue {
            Some(dialogue.id.clone())
        } else {
            None
        };

        ActorParams {
            id: self.id.clone(),
            behavior: self.behavior.clone().into(),
            position: Some(self.body.position),
            name: self.name.clone(),
            strength: self.stats.strength,
            dexterity: self.stats.dexterity,
            constitution: self.stats.constitution,
            intelligence: self.stats.intelligence,
            willpower: self.stats.willpower,
            perception: self.stats.perception,
            charisma: self.stats.charisma,
            current_health: self.stats.current_health,
            current_stamina: self.stats.current_stamina,
            current_energy: self.stats.current_energy,
            factions: self.factions.clone(),
            collider: self.body.collider,
            inventory: self.inventory.to_params(),
            equipped_items: self.equipped_items.clone(),
            animation_player: self.animation_player.clone().into(),
            experience: self.experience,
            can_level_up: self.can_level_up,
            dialogue_id,
        }
    }

    pub fn from_export(game_state: Handle<GameState>, position: Vec2, controller_kind: ActorControllerKind, export: SavedCharacter) -> Self {
        let resources = storage::get::<Resources>();

        let active_missions = export.active_missions
            .into_iter()
            .map(|mission_id| {
                let params = resources.missions.get(&mission_id).cloned().unwrap();
                Mission::new(params)
            })
            .collect();

        let completed_missions = export.completed_missions
            .into_iter()
            .map(|mission_id| {
                let params = resources.missions.get(&mission_id).cloned().unwrap();
                Mission::new(params)
            })
            .collect();

        let body = PhysicsBody::new(position, 0.0, export.actor.collider);

        let dialogue = if let Some(dialogue_id) = &export.actor.dialogue_id {
            resources.dialogue.get(dialogue_id).cloned()
        } else {
            None
        };

        let mode = Box::new(TestMode {});

        Actor {
            id: export.actor.id.clone(),
            name: export.actor.name.clone(),
            active_missions,
            completed_missions,
            noise_level: NoiseLevel::None,
            behavior: export.actor.behavior.clone(),
            stats: export.actor.clone().into(),
            factions: export.actor.factions,
            body,
            inventory: Inventory::from_saved(&export.actor.inventory, &export.items),
            equipped_items: export.actor.equipped_items,
            primary_ability: None,
            secondary_ability: None,
            controller: ActorController::new(controller_kind),
            experience: export.actor.experience,
            dialogue,
            current_dialogue: None,
            animation_player: SpriteAnimationPlayer::new(export.actor.animation_player),
            noise_level_timer: 0.0,
            can_level_up: export.actor.can_level_up,
            automaton: ActorBehaviorFamily::automaton_with_mode(mode),
            game_state,
        }
    }

    pub fn to_export(&self) -> SavedCharacter {
        let game_params = storage::get::<GameParams>();
        let actor = self.to_params();
        let items = self.inventory.items
            .iter()
            .map(|entry| entry.params.clone())
            .collect();

        let active_missions = self.active_missions
            .iter()
            .map(|mission| mission.id.clone())
            .collect();

        let completed_missions = self.completed_missions
            .iter()
            .map(|mission| mission.id.clone())
            .collect();

        let current_chapter = storage::get::<SceneTransitionParams>();

        SavedCharacter {
            game_version: game_params.game_version.clone(),
            actor,
            items,
            active_missions,
            completed_missions,
            current_chapter_index: current_chapter.chapter_index,
            current_map_id: current_chapter.map_id.clone(),
        }
    }

    pub fn take_damage(&mut self, actor_id: &str, _damage_type: DamageType, amount: f32) {
        self.behavior.last_attacked_by = Some(actor_id.to_string());
        self.stats.current_health -= amount;
    }

    pub fn apply_effect(&mut self, actor_id: &str, factions: &[String], effect: Effect) -> bool {
        match effect {
            Effect::Damage { damage_type, amount } => {
                if actor_id != self.id {
                    for faction in factions {
                        if self.factions.contains(faction) {
                            return false;
                        }
                    }
                    self.take_damage(actor_id, damage_type, amount);
                    return true;
                }
                return false;
            }
        }
    }

    pub fn find_by_player_id(id: &str) -> Option<RefMut<Self>> {
        for actor in scene::find_nodes_by_type::<Self>() {
            match &actor.controller.kind {
                ActorControllerKind::LocalPlayer { player_id } => {
                    if player_id == id {
                        return Some(actor);
                    }
                }
                ActorControllerKind::RemotePlayer { player_id } => {
                    if player_id == id {
                        return Some(actor);
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn find_by_id(id: &str) -> Option<RefMut<Actor>> {
        for actor in scene::find_nodes_by_type::<Actor>() {
            if actor.id == id.to_string() {
                return Some(actor);
            }
        }
        None
    }

    pub fn is_local_player(&self) -> bool {
        match &self.controller.kind {
            ActorControllerKind::LocalPlayer { player_id: _ } => true,
            _ => false,
        }
    }

    pub fn set_animation(&mut self, direction: Vec2, is_stationary: bool) {
        if direction.x > 0.0 && direction.x.abs() > direction.y.abs() {
            self.animation_player.start_animation(2);
            self.animation_player.flip_x = false;
        } else if direction.x < 0.0 {
            self.animation_player.start_animation(2);
            self.animation_player.flip_x = true;
        } else if direction.y > 0.0 && direction.y.abs() > direction.x.abs() {
            self.animation_player.start_animation(0);
        } else if direction.y < 0.0 {
            self.animation_player.start_animation(1);
        } else {
            self.animation_player.set_frame(1);
            self.animation_player.stop();
        }
        if is_stationary {
            self.animation_player.set_frame(1);
            self.animation_player.stop();
        }
    }

    pub fn set_noise_level(&mut self, level: NoiseLevel) {
        self.noise_level_timer = 0.0;
        if self.noise_level < level {
            self.noise_level = level;
        }
    }

    pub fn is_target_visible(&self, target: Vec2) -> bool {
        self.body.position.distance(target) <= self.stats.view_distance
            && self.body.raycast(target, true).is_none()
    }

    pub fn add_experience(&mut self, amount: u32) {
        if self.can_level_up {
            self.experience += amount;
        }
    }

    fn update_controller(&mut self) {
        let controller_kind = self.controller.kind.clone();
        match controller_kind {
            ActorControllerKind::LocalPlayer { player_id } => {
                apply_input(&player_id, self);
            }
            ActorControllerKind::RemotePlayer { player_id: _ } => {}
            ActorControllerKind::Computer => {
                Automaton::next(&mut self.automaton, |mode| mode.update());
            }
            ActorControllerKind::None => {}
        }
    }

    fn update_noise_level(&mut self) {
        self.noise_level_timer += get_frame_time();
        if self.noise_level_timer >= Self::NOISE_LEVEL_COOLDOWN {
            self.noise_level = match self.noise_level {
                NoiseLevel::Extreme => NoiseLevel::Loud,
                NoiseLevel::Loud => NoiseLevel::Moderate,
                NoiseLevel::Moderate => NoiseLevel::Silent,
                _ => NoiseLevel::None,
            };
            self.noise_level_timer = 0.0;
        }
    }

    fn update_missions(&mut self) {
        let mut active_missions = self.active_missions.clone();
        for i in 0..active_missions.len() {
            let mission = active_missions.get_mut(i).unwrap();
            if mission.no_autocompletion == false {
                for objective in &mut mission.objectives {
                    match &objective.0 {
                        MissionObjective::Kill { actor_id } => {
                            let game_state = scene::get_node(self.game_state);
                            if game_state.dead_actors.contains(actor_id) {
                                objective.1 = true;
                            }
                        }
                        MissionObjective::FindItem { item_id } => {
                            if self.inventory.items.iter().find(|entry| entry.params.id == item_id.clone()).is_some() {
                                objective.1 = true;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        let mut completed_missions: Vec<Mission> = active_missions.drain_filter(|mission| {
            for (_, is_completed) in mission.objectives.clone() {
                if is_completed == false {
                    return false;
                }
            }

            if mission.no_autocompletion == false {
                mission.is_completed = true;
            }

            if mission.is_completed {
                let resources = storage::get::<Resources>();
                for reward in &mission.rewards {
                    match reward {
                        MissionReward::Item { prototype_id, amount } => {
                            let params = resources.items.get(prototype_id).unwrap();
                            for _ in 0..*amount {
                                self.inventory.add_item(params.clone());
                            }
                        }
                        MissionReward::Credits { amount } => {
                            self.inventory.add_credits(*amount);
                        }
                        MissionReward::Experience { amount } => {
                            self.add_experience(*amount);
                        }
                    }
                }

                return true;
            }

            false
        }).collect();

        let resources = storage::get::<Resources>();
        for mission in &completed_missions {
            for next_id in mission.next_mission_ids.clone() {
                let params = resources.missions.get(&next_id).cloned().unwrap();
                active_missions.push(Mission::new(params));
            }
        }

        self.active_missions = active_missions;
        self.completed_missions.append(&mut completed_missions);
    }

    pub fn equip_item(&mut self, item_id: &str) {
        let item_id = item_id.to_string();
        let mut found_entry = None;
        for entry in &self.inventory.items {
            if entry.params.id == item_id {
                found_entry = Some(entry.clone());
            }
        }
        let mut slot = EquipmentSlot::None;
        if let Some(entry) = found_entry {
            slot = match entry.params.kind {
                ItemKind::OneHandedWeapon => {
                    if self.equipped_items.main_hand.is_some() && self.equipped_items.off_hand.is_none() {
                        EquipmentSlot::OffHand
                    } else {
                        EquipmentSlot::MainHand
                    }
                }
                ItemKind::TwoHandedWeapon => EquipmentSlot::BothHands,
                _ => EquipmentSlot::None,
            };
            self.unequip_slot(slot.clone());
            match slot {
                EquipmentSlot::MainHand => {
                    self.equipped_items.main_hand = Some(entry.params.id.clone());
                    self.primary_ability = entry.get_actor_ability();
                }
                EquipmentSlot::OffHand => {
                    self.equipped_items.off_hand = Some(entry.params.id.clone());
                    self.secondary_ability = entry.get_actor_ability();
                }
                EquipmentSlot::BothHands => {
                    self.equipped_items.main_hand = Some(entry.params.id.clone());
                    self.equipped_items.off_hand = Some(entry.params.id.clone());
                    self.primary_ability = entry.get_actor_ability();
                }
                EquipmentSlot::None => {}
            }
        }

        if let Some(entry) = self.inventory.items.iter_mut().find(|entry| entry.params.id == item_id) {
            entry.equipped_to = slot;
        }
    }

    pub fn unequip_slot(&mut self, slot: EquipmentSlot) {
        let item_ids = match slot {
            EquipmentSlot::MainHand => {
                if let Some(item_id) = self.equipped_items.main_hand.clone() {
                    vec!(item_id)
                } else {
                    Vec::new()
                }
            }
            EquipmentSlot::OffHand => {
                if let Some(item_id) = self.equipped_items.off_hand.clone() {
                    vec!(item_id)
                } else {
                    Vec::new()
                }
            }
            EquipmentSlot::BothHands => {
                let mut item_ids = Vec::new();
                if let Some(item_id) = self.equipped_items.main_hand.clone() {
                    item_ids.push(item_id);
                }
                if let Some(item_id) = self.equipped_items.off_hand.clone() {
                    item_ids.push(item_id);
                }
                item_ids
            }
            EquipmentSlot::None => Vec::new(),
        };

        for item_id in item_ids {
            self.unequip_item(&item_id);
        }
    }

    pub fn unequip_item(&mut self, item_id: &str) {
        if let Some(found_id) = self.equipped_items.main_hand.clone() {
            if found_id == item_id.to_string() {
                self.equipped_items.main_hand = None;
                self.primary_ability = None;
            }
        }
        if let Some(found_id) = self.equipped_items.off_hand.clone() {
            if found_id == item_id.to_string() {
                self.equipped_items.off_hand = None;
                self.secondary_ability = None;
            }
        }
        if let Some(entry) = self.inventory.items.iter_mut().find(|entry| entry.params.id == item_id.to_string()) {
            entry.equipped_to = EquipmentSlot::None;
        }
    }
}

impl BufferedDraw for Actor {
    fn buffered_draw(&mut self) {
        self.body.debug_draw();
        {
            let (position, rotation) = (self.body.position, self.body.rotation);
            self.animation_player.draw(position, rotation);
        }

        let (position, offset_y, alignment, length, height, border) =
            (self.body.position, Self::HEALTH_BAR_OFFSET_Y, HorizontalAlignment::Center, Self::HEALTH_BAR_LENGTH, Self::HEALTH_BAR_HEIGHT, 1.0);

        if self.is_local_player() == false && self.stats.current_health < self.stats.max_health {
            draw_progress_bar(
                self.stats.current_health,
                self.stats.max_health,
                position + vec2(0.0, offset_y),
                length,
                height,
                color::RED,
                color::GRAY,
                border,
                alignment.clone(),
                None, // Some(&format!("{}/{}", self.stats.current_health.round(), self.stats.max_health.round())),
                None,
            );
        }

        let game_state = scene::get_node(self.game_state);
        if game_state.in_debug_mode {
            if let Some(path) = self.behavior.current_path.clone() {
                let mut previous: Option<Vec2> = None;
                for p2 in path.nodes {
                    if let Some(p1) = previous {
                        draw_line(
                            p1.x,
                            p1.y,
                            p2.x,
                            p2.y,
                            2.0,
                            color::BLUE,
                        );
                    }
                    previous = Some(p2);
                }
            }

            let center_position = if let Some(collider) = self.body.get_offset_collider() {
                collider.get_center()
            } else {
                self.body.position
            };
            // if self.noise_level != NoiseLevel::None {
            //     draw_aligned_text(
            //         &format!("noise level: {}", self.noise_level.to_string()),
            //         center_position.x,
            //         self.body.position.y - 50.0,
            //         HorizontalAlignment::Center,
            //         VerticalAlignment::Center,
            //         TextParams {
            //             ..Default::default()
            //         },
            //     )
            // }
            // if let Some(action) = &self.behavior.current_action {
            //     draw_aligned_text(
            //         action,
            //         center_position.x,
            //         center_position.y + 16.0,
            //         HorizontalAlignment::Center,
            //         VerticalAlignment::Top,
            //         Default::default(),
            //     );
            // }
            // draw_circle_lines(
            //     self.body.position.x,
            //     self.body.position.y,
            //     self.stats.view_distance,
            //     2.0,
            //     color::RED,
            // );
            // if self.noise_level != NoiseLevel::None {
            //     draw_circle_lines(
            //         self.body.position.x,
            //         self.body.position.y,
            //         self.noise_level.to_range(),
            //         2.0,
            //         color::YELLOW,
            //     )
            // }
        }
    }

    fn get_z_index(&self) -> f32 {
        self.body.position.y
    }

    fn get_bounds(&self) -> Bounds {
        if let Some(collider) = self.body.get_offset_collider() {
            Bounds::Collider(collider)
        } else {
            Bounds::Point(self.body.position)
        }
    }
}

impl Node for Actor {
    fn ready(node: RefMut<Self>) {
        let mut draw_buffer = scene::find_node_by_type::<DrawBuffer<Self>>().unwrap();
        draw_buffer.buffered.push(node.handle());

        if node.is_local_player() {
            let mut camera = scene::find_node_by_type::<Camera>().unwrap();
            camera.position = node.body.position;
        }
    }

    fn update(mut node: RefMut<Self>) {
        if node.stats.current_health <= 0.0 {
            let mut game_state = scene::find_node_by_type::<GameState>().unwrap();
            let position = node.body.position;
            node.inventory.drop_all(position, true);
            game_state.dead_actors.push(node.id.clone());
            node.delete();
            return;
        }

        node.stats.update();
        node.animation_player.update();
        node.update_noise_level();
        node.update_missions();

        if let Some(ability) = node.primary_ability.as_mut() {
            ability.update();
        }

        if let Some(ability) = node.secondary_ability.as_mut() {
            ability.update();
        }

        node.update_controller();

        {
            let controller = node.controller.clone();
            node.set_animation(controller.aim_direction, controller.move_direction == Vec2::ZERO);

            if controller.should_use_primary_ability {
                let mut primary_ability = node.primary_ability.clone();
                let position = node.body.position.clone();
                if let Some(ability) = &mut primary_ability {
                    ability.activate(&mut *node, position, controller.aim_direction);
                }
                node.primary_ability = primary_ability;
            }

            if controller.should_use_primary_ability {
                let mut secondary_ability = node.secondary_ability.clone();
                let position = node.body.position.clone();
                if let Some(ability) = &mut secondary_ability {
                    ability.activate(&mut *node, position, controller.aim_direction);
                }
                node.secondary_ability = secondary_ability;
            }
        }

        let collider = Collider::circle(0.0, 0.0, Self::PICK_UP_RADIUS).offset(node.body.position);
        for credits in scene::find_nodes_by_type::<Credits>() {
            if collider.contains(credits.position) {
                node.inventory.credits += credits.amount;
                credits.delete();
            }
        }

        if node.controller.should_pick_up_items {
            for item in scene::find_nodes_by_type::<Item>() {
                if collider.contains(item.position) {
                    node.inventory.pick_up(item);
                }
            }
        }

        if node.controller.should_start_interaction {
            if node.current_dialogue.is_some() {
                node.current_dialogue = None;
            } else {
                let collider = Collider::circle(0.0, 0.0, Self::INTERACT_RADIUS).offset(node.body.position);
                for actor in scene::find_nodes_by_type::<Actor>() {
                    if let Some(other_collider) = actor.body.get_offset_collider() {
                        if collider.overlaps(other_collider) {
                            if let ActorControllerKind::Computer = actor.controller.kind {
                                if actor.controller.is_attacking() == false {
                                    node.current_dialogue = actor.dialogue.clone();
                                    node.controller.should_start_interaction = false; // stop this form firing twice
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        let current_dialogue = node.current_dialogue.clone();
        if let Some(mut dialogue) = current_dialogue {
            if dialogue.should_apply {
                dialogue.should_apply = false;
                dialogue.apply_action(&mut *node);
                node.current_dialogue = Some(dialogue);
            }
        }

        let direction = node.controller.move_direction;
        node.body.velocity = if direction != Vec2::ZERO {
            direction * if node.inventory.get_total_weight() >= node.stats.carry_capacity {
                node.set_noise_level(Self::MOVE_NOISE_LEVEL);
                node.stats.move_speed * Self::ENCUMBERED_SPEED_FACTOR
            } else if node.controller.should_sprint && node.stats.current_stamina >= Self::SPRINT_STAMINA_COST {
                node.set_noise_level(Self::SPRINT_NOISE_LEVEL);
                node.stats.current_stamina -= Self::SPRINT_STAMINA_COST;
                node.stats.move_speed * Self::SPRINT_SPEED_FACTOR
            } else {
                node.set_noise_level(Self::MOVE_NOISE_LEVEL);
                node.stats.move_speed
            }
        } else {
            Vec2::ZERO
        };

        node.body.integrate();
    }
}
