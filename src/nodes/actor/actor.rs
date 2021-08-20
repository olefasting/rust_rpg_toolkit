use std::ops::Sub;

use macroquad::{
    color,
    experimental::{
        scene::{
            Handle,
            Node,
            RefMut,
        },
        collections::storage,
    },
    prelude::*,
};

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    render::draw_aligned_text,
    generate_id,
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
        Viewport,
    },
    ability::Ability,
    Resources,
    input::apply_local_input,
    nodes::{
        Item,
        ItemParams,
        draw_buffer::{
            Bounds,
            BufferedDraw,
            DrawBuffer,
        },
        actor::{
            ActorControllerKind,
            ActorInventory,
            ActorStats,
            ActorController,
        }
    },
    json,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct ActorParams {
    #[serde(default, alias = "id", skip_serializing_if = "Option::is_none")]
    pub prototype_id: Option<String>,
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
    pub inventory: Vec<String>,
    pub animation_player: SpriteAnimationParams,
}

impl Default for ActorParams {
    fn default() -> Self {
        ActorParams {
            prototype_id: None,
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
            inventory: Vec::new(),
            animation_player: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct Actor {
    pub id: String,
    pub name: String,
    pub stats: ActorStats,
    pub factions: Vec<String>,
    pub body: PhysicsBody,
    pub inventory: ActorInventory,
    pub primary_ability: Option<Ability>,
    pub secondary_ability: Option<Ability>,
    pub controller: ActorController,
    animation_player: SpriteAnimationPlayer,
}

impl Actor {
    const HEALTH_BAR_LENGTH: f32 = 50.0;
    const HEALTH_BAR_HEIGHT: f32 = 10.0;
    const HEALTH_BAR_OFFSET_Y: f32 = 25.0;

    const ENCUMBERED_SPEED_FACTOR: f32 = 0.1;

    const SPRINT_SPEED_FACTOR: f32 = 2.0;
    const SPRINT_STAMINA_COST: f32 = 10.0;

    const PICK_UP_RADIUS: f32 = 36.0;
    const INTERACT_RADIUS: f32 = 36.0;

    pub fn new(controller_kind: ActorControllerKind, params: ActorParams) -> Self {
        let resources = storage::get::<Resources>();
        let item_params: Vec<ItemParams> = params.inventory
            .iter()
            .map(|prototype_id| resources.items.get(prototype_id)
                .cloned()
                .expect(&format!("Unable to load item prototype with id '{}'!", prototype_id)))
            .collect();

        Actor {
            id: generate_id(),
            name: params.name.clone(),
            stats: ActorStats::from(&params),
            factions: params.factions,
            body: PhysicsBody::new(params.position.unwrap_or_default(), 0.0, params.collider),
            inventory: ActorInventory::from(item_params),
            primary_ability: None,
            secondary_ability: None,
            controller: ActorController::new(controller_kind),
            animation_player: SpriteAnimationPlayer::new(params.animation_player.clone()),
        }
    }

    pub fn add_node(controller_kind: ActorControllerKind, params: ActorParams) -> Handle<Self> {
        scene::add_node(Self::new(controller_kind, params))
    }

    pub fn take_damage(&mut self, _actor_id: &str, damage: f32) {
        self.stats.current_health -= damage;
    }

    pub fn find_by_player_id(id: &str) -> Option<RefMut<Self>> {
        for actor in scene::find_nodes_by_type::<Self>() {
            match &actor.controller.kind {
                ActorControllerKind::LocalPlayer { player_id } => {
                    if player_id == id {
                        return Some(actor);
                    }
                },
                ActorControllerKind::RemotePlayer { player_id } => {
                    if player_id == id {
                        return Some(actor)
                    }
                },
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

    pub fn is_local_player(&self) -> bool {
        match &self.controller.kind {
            ActorControllerKind::LocalPlayer { player_id: _ } => true,
            _ => false,
        }
    }

    pub fn interact(&self, other: &mut Actor) {
        println!("INTERACTION between '{}' and '{}'", self.name, other.name);
    }
}

impl Into<ActorParams> for Actor {
    fn into(self) -> ActorParams {
        ActorParams {
            prototype_id: None,
            position: Some(self.body.position),
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
            inventory: self.inventory
                .to_params()
                .into_iter()
                .filter_map(|params| params.prototype_id)
                .collect(),
            animation_player: SpriteAnimationParams::from(self.animation_player),
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
    }

    fn update(mut node: RefMut<Self>) {
        node.stats.update();
        node.animation_player.update();

        if node.stats.current_health <= 0.0 {
            let position = node.body.position;
            node.inventory.drop_all(position);
            node.delete();
            return;
        }

        if let Some(ability) = node.primary_ability.as_mut() {
            ability.update();
        }

        if let Some(ability) = node.secondary_ability.as_mut() {
            ability.update();
        }

        let controller_kind = node.controller.kind.clone();
        match controller_kind {
            ActorControllerKind::LocalPlayer { player_id } => {
                apply_local_input(&player_id, &mut node.controller);
            }
            ActorControllerKind::RemotePlayer { player_id: _ } => {

            }
            ActorControllerKind::Computer => {
                // TODO: Computer controlled
            }
            ActorControllerKind::None => {}
        }

        let controller_direction = node.controller.direction;
        if let Some(target) = node.controller.primary_target {
            let direction = target.sub(node.body.position).normalize_or_zero();
            node.set_animation(direction, controller_direction == Vec2::ZERO);
        } else if let Some(target) = node.controller.secondary_target {
            let direction = target.sub(node.body.position).normalize_or_zero();
            node.set_animation(direction, controller_direction == Vec2::ZERO);
        } else {
            node.set_animation(controller_direction, false);
        }

        let controller = node.controller.clone();
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
    }

    fn fixed_update(mut node: RefMut<Self>) {
        let direction = node.controller.direction.normalize_or_zero();
        node.body.velocity = direction * if node.inventory.get_total_weight() >= node.stats.carry_capacity {
            node.stats.move_speed * Self::ENCUMBERED_SPEED_FACTOR
        } else if node.controller.is_sprinting && node.stats.current_stamina >= Self::SPRINT_STAMINA_COST {
            if direction != Vec2::ZERO {
                node.stats.current_stamina -= Self::SPRINT_STAMINA_COST;
            }
            node.stats.move_speed * Self::SPRINT_SPEED_FACTOR
        } else {
            node.stats.move_speed
        };

        node.body.integrate();

        if node.controller.is_picking_up_items {
            let collider = Collider::circle(0.0, 0.0, Self::PICK_UP_RADIUS).offset(node.body.position);
            for item in scene::find_nodes_by_type::<Item>() {
                if collider.contains(item.position) {
                    node.inventory.pick_up(item);
                }
            }
        }

        if node.controller.is_interacting {
            let collider = Collider::circle(0.0, 0.0, Self::INTERACT_RADIUS).offset(node.body.position);
            for actor in scene::find_nodes_by_type::<Actor>() {
                if let Some(other_collider) = actor.body.get_offset_collider() {
                    if collider.overlaps(other_collider) {
                        for faction in &node.factions {
                            if actor.factions.contains(faction) {
                                actor.interact(&mut *node);
                                node.controller.is_interacting = false; // stop this form firing twice
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

impl BufferedDraw for Actor {
    fn buffered_draw(&mut self) {
        self.body.debug_draw();
        let (position, rotation) = (self.body.position, self.body.rotation);
        self.animation_player.draw(position, rotation);

        let is_local_player = self.is_local_player();
        let (position, offset_y, alignment, length, height, border) = if is_local_player {
            let viewport = storage::get::<Viewport>();
            let height = Self::HEALTH_BAR_HEIGHT * viewport.scale;
            (vec2(10.0, 10.0), height / 2.0, HorizontalAlignment::Left, Self::HEALTH_BAR_LENGTH * viewport.scale, height, viewport.scale)
        } else {
            (self.body.position, Self::HEALTH_BAR_OFFSET_Y, HorizontalAlignment::Center, Self::HEALTH_BAR_LENGTH, Self::HEALTH_BAR_HEIGHT, 1.0)
        };
        if is_local_player || self.stats.current_health < self.stats.max_health {
            if is_local_player {
                push_camera_state();
                set_default_camera();
            }
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
        if is_local_player {
            draw_aligned_text(
                &format!("position: {}", self.body.position.to_string()),
                screen_width() - 50.0,
                50.0,
                HorizontalAlignment::Right,
                Default::default(),
            );
            draw_progress_bar(
                self.stats.current_stamina,
                self.stats.max_stamina,
                position + vec2(0.0, offset_y + height),
                length,
                height,
                color::YELLOW,
                color::GRAY,
                border,
                alignment.clone(),
                None, // Some(&format!("{}/{}", self.stats.current_stamina.round(), self.stats.max_stamina.round())),
                None,
            );
            draw_progress_bar(
                self.stats.current_energy,
                self.stats.max_energy,
                position + vec2(0.0, offset_y + height * 2.0),
                length,
                height,
                color::BLUE,
                color::GRAY,
                border,
                alignment,
                None, // Some(&format!("{}/{}", self.stats.current_energy.round(), self.stats.max_energy.round())),
                None,
            );
            pop_camera_state();
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
