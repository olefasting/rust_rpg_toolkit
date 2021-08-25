# Dialogue

Actor dialogue can be defined by editing `assets/dialogue.json` and added to an actor by referencing the id of the root option in the tree in the `dialogue` field of an actor definition in `assets/actors.json`.

Every entry in the dialogue definition file needs to have an `id`. Except for that, all other fields are optional. It doesn't make much sense to leave out `title`, `body` and `response`, however, unless you are defining a root dialogue (these root options are also the reason why many fields are left optional).

```rust
struct ActorDialogue {
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
    pub requirements: Vec<ActorDialogueRequirement>,
    #[serde(default)]
    pub exclusions: Vec<ActorDialogueRequirement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ActorDialogueAction>,
    #[serde(skip)]
    pub actor_name: String,
}
```

In short, `title` defines the text that shows up in the list of options, when the dialogue option is available as a response (and therefore has no effect on a root option).
The `body` field shows the text body of the dialogue option, spoken by the player. The `response` field holds the actor that holds this dialogue options response to the player.
Both these fields are `Vec<String>`, with every contained `String` representing a line of text (no automatic formatting or line breaking, as of now, so be careful not to overflow, by line breaking properly).

The `requirements` field holds a set of requirements that must be met for this option to show up in the list of available options. The options in `ActorDialogueRequirements` are self-explanatory:

```rust
enum ActorDialogueRequirement {
    #[serde(rename = "active_mission")]
    ActiveMission { mission_id: String },
    #[serde(rename = "completed_mission")]
    CompletedMission { mission_id: String },
    #[serde(rename = "is_in_faction")]
    IsInFaction { faction_id: String },
}
```

The `exclusions` field holds a set of exclusive `ActorDialogueRequirement` entries that will exclude the option from the list of available options, if any of them are true.

In the `action` field we define an optional action that will be applied on the `player` when the dialogue fires:

```rust
enum ActorDialogueAction {
    #[serde(rename = "start_mission")]
    StartMission { mission_id: String },
    #[serde(rename = "complete_mission")]
    CompleteMission { mission_id: String },
}
```

Currently, this is limited to starting and completing a mission but more, like open trade, give item, take item, will be added as we go.

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
