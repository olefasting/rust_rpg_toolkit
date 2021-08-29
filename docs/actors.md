# Actors

An `Actor` node can be created using `ActorParams`, either defined in code or with `JSON` in a game's data files.

```rust
struct ActorParams {
    pub id: String,
    pub name: String,
    pub factions: Vec<String>,
    #[serde(default)]
    pub behavior: ActorBehaviorParams,
    #[serde(default)]
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
    #[serde(default)]
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
    #[serde(default, rename = "dialogue")]
    pub dialogue_id: Option<String>,
}
```

The `id` field will mean different times, depending on the intended use for the `ActorParams` struct.
As mentioned, actor nodes will be instantiated from `ActorParams`, so if editing a game or module data-file, it will
be a prototype id, which can be referenced in a map's spawn points layer, under a spawn points `prototype_id` property,
to spawn that specific prototype. When instantiated, an `Actor` node will have its id randomly generated, unless you
specify an `instance_id` on the spawn point on the map (used for actors that must be killed as a quest objective, for example).
So, if you are editing a saved character file, for example, you will probably see an `id` field that holds a unique, randomly 
generated `String`.

In the `factions` vector, any factions that the actor belong to can be defined by a faction's unique `String` id.

The `behavior` field hold the parameters for an actor's AI behavior:

```rust
struct ActorBehaviorParams {
    pub aggression: ActorAggression,
    #[serde(default)]
    pub home: Option<Vec2>,
    #[serde(default)]
    pub is_stationary: bool,
    #[serde(default)]
    pub is_on_guard: bool,
    #[serde(default)]
    pub flee_at_health_factor: f32,
}
```

The `aggression` field defines how a computer controlled actor will react to other actors
of different factions, as well as how they react to being attacked, and so on. A `home` location can be defined and it will 
determine where the actor will return to, if he, for some reason moves away from his position. If no `home` is defined, an actor's
spawn point will be automatically assigned as its `home`. The `is_stationary`field defines whether an actor will wander around
or if he will stay at his `home` location, only to move away if factors, like being attacked or seeing an enemy, force it to take
action. Another factor that controls what might trigger such an action, is the `is_on_guard` field. If this is set to `true`, an
actor will react to loud sounds nearby, by trying to investigate what caused them.
An actor's `current_health` will be multiplied by `flee_at_health_factor` and the result will be the threshold at which an actor
flees from combat. If it is set to `0`, the actor will never flee.

The `position` field will only apply if spawning an actor from code. When actors are spawned from prototypes, based on a spawn
point defined on a map, the position will be set to the spawn point position and any value in the `position` field discarded.

All of an actor's primary stats, `strength`, `dexterity`, `constitution`, `intelligence`, `willpower` and `charisma`, are what
determines all the derived attributes, like `maximum_health`, `carry_capacity`, `move_speed`, as well as the view distance and,
in the future, the range at which they can react to another actor's `NoiseLevel`. When instantiating n actor node, these will be 
contained in an `ActorStats` struct, under the nodes `stats` field.

```rust
struct ActorStats {
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub intelligence: u32,
    pub willpower: u32,
    pub perception: u32,
    pub charisma: u32,
    pub current_health: f32,
    pub max_health: f32,
    pub current_stamina: f32,
    pub max_stamina: f32,
    pub current_energy: f32,
    pub max_energy: f32,
    pub health_regen: f32,
    pub stamina_regen: f32,
    pub energy_regen: f32,
    pub view_distance: f32,
    pub carry_capacity: f32,
    pub move_speed: f32,
}
```

Except for the already mentioned primary attributes, these are all derived attributes, meaning that they are automatically 
calculated, based on the the primary abilities. An actors current vitals, like `current_health`, can also be set in `ActorParams`,
but, in general, an actor will be instantiated at full health.

The `collider` field holds the parameters for an actor's collider, used for everything from collision detection and resolution
on the map, as well as with projectiles and other effects and whether an actor is within the view frustum, when culling drawn
objects out of the camera's view cone. If no `Collider` is defined, an actor's position will, in general, be used for such calculations.

The various types of `Collider` shapes, as well as their other parameters, will be explained more thoroughly in the documentation
for the physics system.

If defining the `ActorParams` in code, there will be a field named `inventory`, which holds an `InventoryParams` struct;

```rust
struct InventoryParams {
    pub items: Vec<String>,
    #[serde(default)]
    pub credits: u32,
}
```

This struct is flattened when serializing an actor's parameters, so when defining `ActorParams` in JSON, the `items` and `credits` fields
will be located by the root object. As with an actors `id`, the items vector will hold different types of identifiers, 
depending on what purpose of the `ActorParams` struct. When defining actor prototypes in a game's or a module's data files,
the identifiers will be item prototype id's, referencing definitions in the game's or any loaded module's data files.
When an actor is saved as a `SavedCharacter`, these identifiers will be instance id's, referencing items saved in the 
`SavedCharacter` items vector. What items will be equipped by an instantiated actor, as well as the slot they are equipped
to, is determined by the `EquippedItems` struct, in the `equipped_item` field.

An actor's sprites and animations are defined in the `SpriteAnimationParams` in the `animation_player` field. Refer to the
rendering documentation for more info on this.

If `can_level_up` is set to true, the actor can receive experience points and level up and the `dialogue_id` holds an
identifier that references a root `Dialogue` in a game's or a module's data files. For more information on dialogues,
refer to the dialogue documentation.
