# Dialogue

Actor dialogue can be defined by editing `assets/dialogue.json` and added to an actor by referencing the `id` of the root option in the tree in the `dialogue` field of an actor definition in `assets/actors.json`.

Every entry in the dialogue definition file needs to have an `id`. Except for that, all other fields are optional. It doesn't make much sense to leave out `title`, `body` and `response`, however, unless you are defining a root dialogue (these root options are also the reason why many fields are left optional).

```rust
struct Dialogue {
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
}
```

In short, `title` defines the text that shows up in the list of options, when the dialogue option is available as a response (and therefore has no effect on a root option).
The `body` field shows the text body of the dialogue option, spoken by the player and the `response` field holds the response to the player, from the actor that holds the dialogue option.
Both these fields are `Vec<String>`, with every contained `String` representing a line of text (no automatic formatting or line breaking, as of now, so be careful not to overflow, by line breaking properly).

In the `options` field we define available responses by referencing the `id` of other dialogue entries.

The `requirements` field holds a set of requirements that must be met for this option to show up in the list of available options. The options in `ActorDialogueRequirements` are self-explanatory:

```rust
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum DialogueRequirement {
    ActiveMission { mission_id: String },
    CompletedMission { mission_id: String },
    IsInFaction { faction_id: String },
}
```

The `exclusions` field holds a set of exclusive `ActorDialogueRequirement` entries that will exclude the option from the list of available options, if any of them are true.

In the `action` field we define an optional action that will be applied on the `player` when the dialogue fires:

```rust
#[serde(tag = "type", rename_all = "snake_case")]
enum DialogueAction {
    OpenTrade,
    StartMission { mission_id: String },
    CompleteMission { mission_id: String },
    MapTransition { map_id: String },
    CompleteChapter,
}
```
