pub use std::{
    collections::HashMap,
    iter::FromIterator,
    ops::{
        Add,
        Sub,
        Mul,
        Div,
    },
};

pub(crate) use std::{
    fs,
    io,
    path::{
        Path,
        PathBuf,
    },
};

pub(crate) use macroquad::{
    self,
    experimental::{
        animation::{
            AnimatedSprite,
            Animation,
        },
    },
    audio::{
        load_sound,
        set_sound_volume,
    },
    prelude::*,
};

pub use macroquad::{
    experimental::{
        collections::storage,
        scene::{
            Node,
            RefMut,
            Handle,
        },
        coroutines::start_coroutine,
    },
    audio::{
        Sound,
        play_sound,
        play_sound_once,
    },
    window::Conf as WindowConf,
    color,
    math::*,
};

pub(crate) use serde::{
    Serialize,
    Deserialize,
};

pub(crate) use serde_json;

pub use gilrs::GamepadId;

pub use mode::{
    self,
    Automaton,
    Mode,
};

pub(crate) use crate::{
    scene::{
        load_scene,
    },
    input::{
        update_input,
    },
    gui::draw_gui,
    modules::load_modules,
};

pub use crate::{
    versions::{
        get_toolkit_version,
        to_int_version,
        check_version,
    },
    config::Config,
    resources::Resources,
    noise_level::NoiseLevel,
    chapter::{
        Chapter,
        ChapterParams,
    },
    error::{
        Error,
        Result,
    },
    player::{
        LocalPlayer,
        get_player_actor,
    },
    character::{
        Character,
        character_name_to_path,
        get_available_characters,
        load_character,
        delete_character,
    },
    physics::{
        Collider,
        PhysicsBody,
        CollisionKind,
        raycast,
    },
    inventory::{
        Inventory,
        InventoryParams,
        InventoryEntry,
        EquippedItems,
        EquipmentSlot,
    },
    math::{
        Circle,
        URect,
        rotate_vector,
        deg_to_rad,
        rad_to_deg,
    },
    behavior_sets::{
        self,
        DEFAULT_BEHAVIOR_SET_ID,
        ActorBehaviorConstructor,
        register_behavior_set,
        get_behavior_set,
    },
    nodes::{
        actor::{
            Actor,
            ActorParams,
            ActorBehaviorParams,
            ActorStats,
            ActorController,
            ActorControllerKind,
            ActorBehavior,
            ActorBehaviorFamily,
            ActorAggression,
        },
        LightSource,
        Camera,
        ContinuousBeams,
        draw_buffer::{
            DrawBuffer,
            BufferedDraw,
            Bounds,
        },
        GameState,
        Hud,
        item::{
            Item,
            ItemKind,
            ItemParams,
        },
        PostProcessing,
        projectiles::{
            Projectiles,
            ProjectileKind,
        },
        Credits,
        MapRenderer,
    },
    events::{
        Event,
        get_next_event,
        dispatch_event,
        handle_event,
        handle_queued_events,
    },
    game::{
        GameParams,
        init,
        begin_frame,
        end_frame,
    },
    scene::{
        DrawStage,
        SceneBuilder,
    },
    gui::{
        self,
        GuiSkins,
        GuiState,
        WindowBuilder,
        MenuBuilder,
        show_main_menu,
    },
    map::{
        Map,
        MapLayer,
        MapLayerKind,
        MapTileset,
        MapObject,
        MapProperty,
        MapTile,
        NavigationPath,
        ObjectLayerKind,
    },
    missions::{
        Mission,
        MissionParams,
        MissionReward,
        MissionObjective,
    },
    dialogue::{
        Dialogue,
        DialogueAction,
        DialogueRequirement,
    },
    helpers::{
        self,
        sort_by_distance,
        remove_filename,
        get_timestamp,
        generate_id,
    },
    render::{
        COLOR_NONE,
        draw_progress_bar,
        draw_aligned_text,
        color_from_hex_string,
        Sprite,
        SpriteAnimationParams,
        SpriteAnimationPlayer,
        Viewport,
        HorizontalAlignment,
        VerticalAlignment,
    },
    ability::{
        Ability,
        AbilityParams,
        AbilityDelivery,
        DamageType,
        Effect,
    },
    input::{
        self,
        get_mouse_position,
        get_mouse_in_world_space,
        apply_input,
        map_gamepad,
        get_mapped_gamepad,
    },
    json,
};
