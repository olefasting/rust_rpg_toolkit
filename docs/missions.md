# Missions

Missions are defined in the file `assets/missions.json` amd cam currently just be activated in actor dialogue. This will change in the future, however.
Every mission needs an `id`, which can be referenced in dialogue, in order to start or complete a mission.
```rust
struct MissionParams {
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
```

The `title` and `description` fields will be used in the missions overview window, when it is implemented. Currently missions are just shown as lists on the hud but this will change in the future.
A list of objectives are defined in the `objectives` vector and when all are completed, the mission will finish by itself, unless `no_autocompletion` is set to `true`.
This might be removed in the future, to be replaced by a check if the `objectives` vector is empty, in order to determine if autocompletion should fire, but for now we are keeping it verbose, like this.

In the `rewards` field we define any rewards that will be given to the `player` upon completion.

```rust
enum MissionReward {
    #[serde(rename = "item")]
    Item { prototype_id: String, amount: u32 },
    #[serde(rename = "credits")]
    Credits { amount: u32 },
    #[serde(rename = "xp", alias = "experience")]
    Experience { amount: u32 },
}
```

The `next_mission_id` field holds and optional `id` of a mission that should be started automatically as this one finishes and marker can be used to define a mission marker for missions that does not have any objectives (markers are automatically added for objectives).
