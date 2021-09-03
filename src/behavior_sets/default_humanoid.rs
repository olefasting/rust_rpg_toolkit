use crate::prelude::*;

pub struct IdleMode;

impl IdleMode {
    pub fn new() -> Box<dyn ActorBehavior> {
        Box::new(IdleMode {})
    }
}

impl Mode for IdleMode {
    type Family = ActorBehaviorFamily;
}

impl ActorBehavior for IdleMode {
    fn update(
        self: Box<Self>,
        params: ActorBehaviorParams,
        factions: &[String],
        stats: ActorStats,
        position: Vec2,
        _: &mut ActorController,
        _: Option<Ability>,
        _: Option<Ability>,
        _: Inventory,
        _: EquippedItems,
    ) -> Box<dyn ActorBehavior> {
        if params.attackers.len() > 0 {
            for (_, attacker) in params.attackers.clone() {
                if let Some(attacker) = scene::try_get_node(attacker) {
                    if position.distance(attacker.body.position) <= stats.view_distance * 0.9 {
                        match params.aggression {
                            ActorAggression::Passive => return FleeMode::new(attacker.handle()),
                            _ => return AttackMode::new(attacker.handle()),
                        }
                    }
                }
            }
        } else if params.aggression == ActorAggression::Aggressive {
            let mut enemies = scene::find_nodes_by_type::<Actor>()
                .into_iter()
                .filter(|actor| position.distance(actor.body.position) <= stats.view_distance)
                .collect::<Vec<RefMut<Actor>>>();

            enemies.sort_by(|a, b|
                sort_by_distance(position, &a.body.position, &b.body.position));

            'actor: for actor in enemies {
                for faction in &actor.factions {
                    if factions.contains(faction) {
                        continue 'actor;
                    }
                }
                if position.distance(actor.body.position) <= stats.view_distance {
                    return AttackMode::new(actor.handle());
                }
            }
        } else if params.aggression == ActorAggression::Passive {
            for actor in scene::find_nodes_by_type::<Actor>() {
                for faction in &actor.factions {
                    if factions.contains(faction) {
                        continue;
                    }
                }
                if position.distance(actor.body.position) <= stats.view_distance {
                    return FleeMode::new(actor.handle());
                }
            }
        } else {
            if params.is_on_guard {
                let mut noisy_actors = scene::find_nodes_by_type::<Actor>()
                    .into_iter()
                    .filter(|actor|
                        actor.noise_level >= NoiseLevel::Moderate
                            && position.distance(actor.body.position) <= actor.noise_level.to_range())
                    .collect::<Vec<RefMut<Actor>>>();

                noisy_actors.sort_by(|a, b|
                    sort_by_distance(position, &a.body.position, &b.body.position));

                for actor in noisy_actors {
                    let distance = position.distance(actor.body.position);
                    // TODO: make hearing distance dependent on PER
                    if distance <= actor.noise_level.to_range() && distance >= stats.view_distance * 0.9 {
                        return InvestigateMode::new(actor.body.position);
                    }
                }
            }

            if params.is_stationary {
                if let Some(home) = params.home {
                    if position.distance(home) > 2.0 {
                        return GoToMode::new(home);
                    }
                }
            } else {
                let game_state = scene::find_node_by_type::<GameState>().unwrap();
                let dist_x = 5.0 * game_state.map.tile_size.x;
                let dist_y = 5.0 * game_state.map.tile_size.y;
                let x = rand::gen_range(position.x - dist_x, position.x + dist_x);
                let y = rand::gen_range(position.y - dist_y, position.y + dist_y);
                return GoToMode::new(vec2(x, y));
            }
        }

        self
    }
}

pub struct GoToMode {
    pub destination: Vec2,
    pub path: Option<NavigationPath>,
}

impl GoToMode {
    pub fn new(destination: Vec2) -> Box<Self> {
        Box::new(GoToMode {
            destination,
            path: None,
        })
    }
}

impl Mode for GoToMode {
    type Family = ActorBehaviorFamily;
}

impl ActorBehavior for GoToMode {
    fn update(
        mut self: Box<Self>,
        _: ActorBehaviorParams,
        _: &[String],
        _: ActorStats,
        position: Vec2,
        controller: &mut ActorController,
        _: Option<Ability>,
        _: Option<Ability>,
        _: Inventory,
        _: EquippedItems,
    ) -> Box<dyn ActorBehavior> {
        if let Some(path) = self.path.clone() {
            if path.destination != self.destination {
                let game_state = scene::find_node_by_type::<GameState>().unwrap();
                self.path = game_state.map.get_path(position, self.destination);
            } else {
                self.path = process_path(position, controller, path);
            }
        } else {
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            self.path = game_state.map.get_path(position, self.destination);
        }

        if self.path.is_none() {
            return IdleMode::new();
        }

        self
    }
}

pub struct AttackMode {
    pub target: Handle<Actor>,
    pub path: Option<NavigationPath>,
}

impl AttackMode {
    pub fn new(target: Handle<Actor>) -> Box<Self> {
        Box::new(AttackMode {
            target,
            path: None,
        })
    }
}

impl Mode for AttackMode {
    type Family = ActorBehaviorFamily;
}

impl ActorBehavior for AttackMode {
    fn update(
        mut self: Box<Self>,
        _: ActorBehaviorParams,
        _: &[String],
        _: ActorStats,
        position: Vec2,
        controller: &mut ActorController,
        primary_ability: Option<Ability>,
        secondary_ability: Option<Ability>,
        _: Inventory,
        _: EquippedItems,
    ) -> Box<dyn ActorBehavior> {
        if let Some(target) = scene::try_get_node(self.target) {
            if primary_ability.is_none() && secondary_ability.is_none() {
                return Box::new(EquipWeaponMode {});
            }

            let distance = position.distance(target.body.position);
            if let Some(ability) = primary_ability {
                if distance <= ability.range * 0.9 {
                    self.path = None;
                    controller.should_use_primary_ability = true;
                } else {
                    self.path = if let Some(path) = self.path.clone() {
                        process_path(position, controller, path)
                    } else {
                        let game_state = scene::find_node_by_type::<GameState>().unwrap();
                        game_state.map.get_path(position, target.body.position)
                    }
                }
            }
            if let Some(ability) = secondary_ability {
                if distance <= ability.range * 0.9 {
                    controller.should_use_secondary_ability = true;
                }
            }
            controller.aim_direction = target.body.position.sub(position).normalize_or_zero();
        } else {
            return IdleMode::new();
        }

        controller.should_sprint = true;
        self
    }
}

pub struct FleeMode {
    pub from: Handle<Actor>,
}

impl FleeMode {
    pub fn new(from: Handle<Actor>) -> Box<Self> {
        Box::new(FleeMode {
            from,
        })
    }
}

impl Mode for FleeMode {
    type Family = ActorBehaviorFamily;
}

impl ActorBehavior for FleeMode {
    fn update(
        self: Box<Self>,
        _: ActorBehaviorParams,
        _: &[String],
        stats: ActorStats,
        position: Vec2,
        controller: &mut ActorController,
        _: Option<Ability>,
        _: Option<Ability>,
        _: Inventory,
        _: EquippedItems,
    ) -> Box<dyn ActorBehavior> {
        if let Some(from) = scene::try_get_node(self.from) {
            if position.distance(from.body.position) <= stats.view_distance {
                controller.move_direction = position.sub(from.body.position).normalize_or_zero();
                controller.should_sprint = true;
                return self;
            }
        }

        IdleMode::new()
    }
}

pub struct InvestigateMode {
    pub location: Vec2,
    pub path: Option<NavigationPath>,
}

impl InvestigateMode {
    pub fn new(location: Vec2) -> Box<Self> {
        Box::new(InvestigateMode {
            location,
            path: None,
        })
    }
}

impl Mode for InvestigateMode {
    type Family = ActorBehaviorFamily;
}

impl ActorBehavior for InvestigateMode {
    fn update(
        mut self: Box<Self>,
        _: ActorBehaviorParams,
        _: &[String],
        stats: ActorStats,
        position: Vec2,
        controller: &mut ActorController,
        _: Option<Ability>,
        _: Option<Ability>,
        _: Inventory,
        _: EquippedItems,
    ) -> Box<dyn ActorBehavior> {
        if position.distance(self.location) <= stats.view_distance * 0.9 {
            return IdleMode::new();
        }

        self.path = if let Some(path) = self.path {
            process_path(position, controller, path)
        } else {
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            game_state.map.get_path(position, self.location)
        };

        controller.should_sprint = true;
        self
    }
}

pub struct EquipWeaponMode;

impl EquipWeaponMode {
    pub fn new() -> Box<Self> {
        Box::new(EquipWeaponMode {})
    }
}

impl Mode for EquipWeaponMode {
    type Family = ActorBehaviorFamily;
}

impl ActorBehavior for EquipWeaponMode {
    fn update(
        self: Box<Self>,
        _: ActorBehaviorParams,
        _: &[String],
        _: ActorStats,
        _: Vec2,
        controller: &mut ActorController,
        _: Option<Ability>,
        _: Option<Ability>,
        inventory: Inventory,
        _: EquippedItems,
    ) -> Box<dyn ActorBehavior> {
        if let Some(weapon) = inventory
            .get_all_of_kind(&[ItemKind::TwoHandedWeapon, ItemKind::OneHandedWeapon])
            .first() {
            controller.equip_weapon = Some(weapon.params.id.clone());
        }

        IdleMode::new()
    }
}

fn process_path(position: Vec2, controller: &mut ActorController, mut path: NavigationPath) -> Option<NavigationPath> {
    if let Some(mut node) = path.nodes.first().cloned() {
        if position.distance(node) <= 2.0 {
            path.nodes.remove(0);
            if let Some(next) = path.nodes.first().cloned() {
                node = next;
            } else {
                return None;
            }
        }
        controller.move_direction = node.sub(position).normalize_or_zero();
        return Some(path);
    }
    None
}
