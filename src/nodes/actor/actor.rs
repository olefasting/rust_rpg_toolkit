use std::{
    ops::Sub,
    cmp::Ordering,
};

use macroquad::{
    color,
    experimental::{
        collections::storage,
        scene::{
            Handle,
            Node,
            RefMut,
        },
    },
    prelude::*,
};

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    ability::Ability,
    generate_id,
    input::apply_local_player_input,
    json,
    nodes::{
        draw_buffer::{
            Bounds,
            BufferedDraw,
            DrawBuffer,
        },
        Item,
        GameState,
        Camera,
    },
    physics::{
        Collider,
        PhysicsBody,
        PhysicsObject,
    },
    render::{
        draw_progress_bar,
        HorizontalAlignment,
        SpriteAnimationParams,
        SpriteAnimationPlayer,
        draw_aligned_text,
        VerticalAlignment,
    },
    missions::{
        Mission,
    },
    Resources,
};

use super::{
    ActorController,
    ActorControllerKind,
    ActorInventory,
    ActorStats,
    ActorBehavior,
    ActorBehaviorParams,
    ActorInventoryParams,
    apply_actor_behavior,
};
use crate::{
    missions::{
        MissionReward,
        MissionObjective,
    },
    nodes::{
        item::Credits
    },
};
use crate::nodes::actor::equipped::{EquippedItems, EquipmentSlot};
use crate::nodes::item::ItemKind;
use crate::ability::{Effect, DamageType};
use crate::dialogue::Dialogue;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum ActorNoiseLevel {
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

impl ActorNoiseLevel {
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

impl Default for ActorNoiseLevel {
    fn default() -> Self {
        Self::None
    }
}

impl ToString for ActorNoiseLevel {
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

impl Ord for ActorNoiseLevel {
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

#[derive(Clone, Serialize, Deserialize)]
pub struct ActorParams {
    #[serde(default, rename = "id", skip_serializing_if = "Option::is_none")]
    pub prototype_id: Option<String>,
    #[serde(default)]
    pub behavior: ActorBehaviorParams,
    #[serde(default, with = "json::opt_vec2", skip_serializing_if = "Option::is_none")]
    pub position: Option<Vec2>,
    pub name: String,
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
    pub factions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collider: Option<Collider>,
    #[serde(default, flatten)]
    pub inventory: ActorInventoryParams,
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
            prototype_id: None,
            behavior: Default::default(),
            position: None,
            name: "Unnamed Actor".to_string(),
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
            factions: Vec::new(),
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

#[derive(Clone)]
pub struct Actor {
    pub id: String,
    pub name: String,
    pub active_missions: Vec<Mission>,
    pub completed_missions: Vec<Mission>,
    pub noise_level: ActorNoiseLevel,
    pub behavior: ActorBehavior,
    pub stats: ActorStats,
    pub factions: Vec<String>,
    pub body: PhysicsBody,
    pub inventory: ActorInventory,
    pub equipped_items: EquippedItems,
    pub primary_ability: Option<Ability>,
    pub secondary_ability: Option<Ability>,
    pub controller: ActorController,
    pub experience: u32,
    pub dialogue: Option<Dialogue>,
    pub current_dialogue: Option<Dialogue>,
    animation_player: SpriteAnimationPlayer,
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

    const MOVE_NOISE_LEVEL: ActorNoiseLevel = ActorNoiseLevel::Silent;
    const SPRINT_NOISE_LEVEL: ActorNoiseLevel = ActorNoiseLevel::Moderate;

    const NOISE_LEVEL_COOLDOWN: f32 = 1.5;

    const PICK_UP_RADIUS: f32 = 36.0;
    const INTERACT_RADIUS: f32 = 36.0;

    pub fn new(instance_id: Option<String>, controller_kind: ActorControllerKind, params: ActorParams) -> Self {
        let position = params.position.unwrap_or_default();
        let dialogue = if let Some(dialogue_id) = params.dialogue_id.clone() {
            let resources = storage::get::<Resources>();
            let mut dialogue = resources.dialogue.get(&dialogue_id).cloned().unwrap();
            dialogue.actor_name = params.name.clone();
            Some(dialogue)
        } else {
            None
        };

        Actor {
            id: instance_id.unwrap_or(generate_id()).to_string(),
            name: params.name.clone(),
            active_missions: Vec::new(),
            completed_missions: Vec::new(),
            noise_level: ActorNoiseLevel::None,
            behavior: ActorBehavior::new(ActorBehaviorParams {
                home: Some(position),
                ..params.behavior
            }),
            stats: ActorStats::from(&params),
            factions: params.factions,
            body: PhysicsBody::new(position, 0.0, params.collider),
            inventory: ActorInventory::from(params.inventory),
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
        }
    }

    pub fn add_node(instance_id: Option<String>, controller_kind: ActorControllerKind, params: ActorParams) -> Handle<Self> {
        scene::add_node(Self::new(instance_id, controller_kind, params))
    }

    pub fn take_damage(&mut self, actor_id: &str, _damage_type: DamageType, amount: f32) {
        self.behavior.last_attacked_by = Some((0.0, actor_id.to_string()));
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

    pub fn set_noise_level(&mut self, level: ActorNoiseLevel) {
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

    fn apply_behavior(&mut self) {
        apply_actor_behavior(self)
    }

    fn update_noise_level(&mut self) {
        self.noise_level_timer += get_frame_time();
        if self.noise_level_timer >= Self::NOISE_LEVEL_COOLDOWN {
            self.noise_level = match self.noise_level {
                ActorNoiseLevel::Extreme => ActorNoiseLevel::Loud,
                ActorNoiseLevel::Loud => ActorNoiseLevel::Moderate,
                ActorNoiseLevel::Moderate => ActorNoiseLevel::Silent,
                _ => ActorNoiseLevel::None,
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
                            let game_state = scene::find_node_by_type::<GameState>().unwrap();
                            if game_state.dead_actors.contains(actor_id) {
                                objective.1 = true;
                            }
                        }
                        MissionObjective::FindItem { item_id } => {
                            if self.inventory.items.iter().find(|entry| entry.id == item_id.clone()).is_some() {
                                objective.1 = true;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        let mut completed_missions = active_missions.drain_filter(|mission| {
            if mission.no_autocompletion && mission.is_completed == false {
                return false;
            }
            for (_, is_completed) in &mission.objectives {
                if *is_completed == false {
                    return false;
                }
            }
            true
        }).collect::<Vec<Mission>>();
        let resources = storage::get::<Resources>();
        for mission in &completed_missions {
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
            if entry.id == item_id {
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
                },
                ItemKind::TwoHandedWeapon => EquipmentSlot::BothHands,
                _ => EquipmentSlot::None,
            };
            self.unequip_slot(slot.clone());
            match slot {
                EquipmentSlot::MainHand => {
                    self.equipped_items.main_hand = Some(entry.id.to_string());
                    self.primary_ability = entry.get_actor_ability();
                }
                EquipmentSlot::OffHand => {
                    self.equipped_items.off_hand = Some(entry.id.to_string());
                    self.secondary_ability = entry.get_actor_ability();
                }
                EquipmentSlot::BothHands => {
                    self.equipped_items.main_hand = Some(entry.id.to_string());
                    self.equipped_items.off_hand = Some(entry.id.to_string());
                    self.primary_ability = entry.get_actor_ability();
                }
                EquipmentSlot::None => {}
            }
        }

        if let Some(entry) = self.inventory.items.iter_mut().find(|entry| entry.id == item_id) {
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
            },
            EquipmentSlot::OffHand => {
                if let Some(item_id) = self.equipped_items.off_hand.clone() {
                    vec!(item_id)
                } else {
                    Vec::new()
                }
            },
            EquipmentSlot::BothHands => {
                let mut item_ids = Vec::new();
                if let Some(item_id) = self.equipped_items.main_hand.clone() {
                    item_ids.push(item_id);
                }
                if let Some(item_id) = self.equipped_items.off_hand.clone() {
                    item_ids.push(item_id);
                }
                item_ids
            },
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
        if let Some(entry) = self.inventory.items.iter_mut().find(|entry| entry.id == item_id.to_string()) {
            entry.equipped_to = EquipmentSlot::None;
        }
    }
}

impl Into<ActorParams> for Actor {
    fn into(self) -> ActorParams {
        let dialogue_id = if let Some(dialogue) = self.dialogue {
            Some(dialogue.id.clone())
        } else {
            None
        };

        ActorParams {
            prototype_id: None,
            position: Some(self.body.position),
            behavior: self.behavior.into(),
            name: self.name,
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
            factions: self.factions,
            collider: self.body.collider,
            inventory: self.inventory.to_params(),
            equipped_items: self.equipped_items,
            dialogue_id,
            animation_player: SpriteAnimationParams::from(self.animation_player),
            experience: self.experience,
            can_level_up: self.can_level_up,
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
        let game_state = scene::find_node_by_type::<GameState>().unwrap();
        if game_state.in_debug_mode {
            let center_position = if let Some(collider) = self.body.get_offset_collider() {
                collider.get_center()
            } else {
                self.body.position
            };
            if self.noise_level != ActorNoiseLevel::None {
                draw_aligned_text(
                    &format!("noise level: {}", self.noise_level.to_string()),
                    center_position.x,
                    self.body.position.y - 50.0,
                    HorizontalAlignment::Center,
                    VerticalAlignment::Center,
                    TextParams {
                        ..Default::default()
                    },
                )
            }
            if let Some(action) = &self.behavior.current_action {
                draw_aligned_text(
                    action,
                    center_position.x,
                    center_position.y + 16.0,
                    HorizontalAlignment::Center,
                    VerticalAlignment::Top,
                    Default::default(),
                );
            }
            draw_circle_lines(
                self.body.position.x,
                self.body.position.y,
                self.stats.view_distance,
                2.0,
                color::RED,
            );
            if self.noise_level != ActorNoiseLevel::None {
                draw_circle_lines(
                    self.body.position.x,
                    self.body.position.y,
                    self.noise_level.to_range(),
                    2.0,
                    color::YELLOW,
                )
            }
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
    fn ready(mut node: RefMut<Self>) {
        node.provides::<PhysicsObject>((
            node.handle().untyped(),
            node.handle().lens(|actor| &mut actor.body),
        ));

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

        let controller_kind = node.controller.kind.clone();
        match controller_kind {
            ActorControllerKind::LocalPlayer { player_id: _ } => {
                apply_local_player_input(&mut node);
            }
            ActorControllerKind::RemotePlayer { player_id: _ } => {}
            ActorControllerKind::Computer => {
                node.apply_behavior();
            }
            ActorControllerKind::None => {}
        }

        let controller = node.controller.clone();

        if let Some(target) = controller.primary_target {
            let direction = target.sub(node.body.position).normalize_or_zero();
            node.set_animation(direction, controller.direction == Vec2::ZERO);
        } else if let Some(target) = controller.secondary_target {
            let direction = target.sub(node.body.position).normalize_or_zero();
            node.set_animation(direction, controller.direction == Vec2::ZERO);
        } else {
            node.set_animation(controller.direction, false);
        }

        if let Some(target) = controller.primary_target {
            let mut primary_ability = node.primary_ability.clone();
            let position = node.body.position.clone();
            if let Some(ability) = &mut primary_ability {
                ability.activate(&mut *node, position, target);
            }
            node.primary_ability = primary_ability;
        }
        if let Some(target) = controller.secondary_target {
            let mut secondary_ability = node.secondary_ability.clone();
            let position = node.body.position.clone();
            if let Some(ability) = &mut secondary_ability {
                ability.activate(&mut *node, position, target);
            }
            node.secondary_ability = secondary_ability;
        }

        let collider = Collider::circle(0.0, 0.0, Self::PICK_UP_RADIUS).offset(node.body.position);
        for credits in scene::find_nodes_by_type::<Credits>() {
            if collider.contains(credits.position) {
                node.inventory.credits += credits.amount;
                credits.delete();
            }
        }
        if node.controller.is_picking_up_items {
            for item in scene::find_nodes_by_type::<Item>() {
                if collider.contains(item.position) {
                    node.inventory.pick_up(item);
                }
            }
        }

        if node.controller.is_starting_interaction {
            if node.current_dialogue.is_some() {
                node.current_dialogue = None;
            } else {
                let collider = Collider::circle(0.0, 0.0, Self::INTERACT_RADIUS).offset(node.body.position);
                for actor in scene::find_nodes_by_type::<Actor>() {
                    if let Some(other_collider) = actor.body.get_offset_collider() {
                        if collider.overlaps(other_collider) {
                            if let ActorControllerKind::Computer = actor.controller.kind {
                                if actor.behavior.is_in_combat == false {
                                    node.current_dialogue = actor.dialogue.clone();
                                    node.controller.is_starting_interaction = false; // stop this form firing twice
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
    }

    fn fixed_update(mut node: RefMut<Self>) {
        let direction = node.controller.direction.normalize_or_zero();
        node.body.velocity = if direction != Vec2::ZERO {
            direction * if node.inventory.get_total_weight() >= node.stats.carry_capacity {
                node.set_noise_level(Self::MOVE_NOISE_LEVEL);
                node.stats.move_speed * Self::ENCUMBERED_SPEED_FACTOR
            } else if node.controller.is_sprinting && node.stats.current_stamina >= Self::SPRINT_STAMINA_COST {
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
