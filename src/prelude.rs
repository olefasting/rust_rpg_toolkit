pub use std::{
    collections::HashMap,
    iter::FromIterator,
    ops::{Add, Div, Mul, Sub},
};

pub(crate) use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub(crate) use macroquad::{
    self,
    experimental::animation::AnimatedSprite,
    prelude::{clear_background, next_frame, Camera2D, Texture2D},
};

pub use macroquad::{
    color,
    experimental::{
        animation::Animation,
        collections::storage,
        coroutines::start_coroutine,
        scene::{self, Handle, Node, RefMut},
    },
    math::*,
    prelude::{
        draw_circle, draw_circle_lines, draw_line, draw_rectangle, draw_rectangle_lines, get_fps,
        get_frame_time, get_time, gl_use_default_material, gl_use_material,
        load_ttf_font_from_bytes, measure_text as get_text_measure, pop_camera_state,
        push_camera_state, rand, render_target as new_render_target,
        screen_height as get_screen_height, screen_width as get_screen_width, set_default_camera,
        Color, DrawTextureParams, FilterMode, Font, Image, ImageFormat, MaterialParams,
        PipelineParams, RenderTarget, TextParams, UniformType,
    },
    window::Conf as WindowConf,
};

pub(crate) use serde::{Deserialize, Serialize};

pub(crate) use serde_json;

pub use mode::{self, Automaton, Mode};

pub(crate) use crate::{gui::draw_gui, modules::load_modules, scene::load_scene};

pub use crate::{
    ability::{Ability, AbilityDelivery, AbilityParams, DamageType, Effect},
    audio::{get_volume, load_sound, load_sound_from_bytes, play_sound, Sound, VolumeCategory},
    behavior_sets::{
        self, get_behavior_set, register_behavior_set, ActorBehaviorConstructor,
        DEFAULT_BEHAVIOR_SET_ID,
    },
    chapter::{Chapter, ChapterParams},
    character::{
        character_name_to_path, delete_character, get_available_characters, load_character,
        Character,
    },
    config::Config,
    dialogue::{Dialogue, DialogueAction, DialogueRequirement},
    error::{Error, ErrorKind, Result},
    events::{dispatch_event, get_next_event, handle_event, handle_queued_events, Event},
    file_io::{load_file, load_file_to_string},
    game::{begin_frame, end_frame, init, GameParams},
    gui::{self, show_main_menu, GuiSkins, GuiState, MenuBuilder, WindowBuilder},
    helpers::{self, generate_id, get_timestamp, remove_filename, sort_by_distance},
    input::{self, apply_input, get_mouse_in_world_space, get_mouse_position},
    inventory::{EquipmentSlot, EquippedItems, Inventory, InventoryEntry, InventoryParams},
    json,
    map::{
        Map, MapLayer, MapLayerKind, MapObject, MapProperty, MapTile, MapTileset, NavigationPath,
        ObjectLayerKind,
    },
    math::{deg_to_rad, rad_to_deg, rotate_vector, Circle, URect},
    missions::{Mission, MissionObjective, MissionParams, MissionReward},
    nodes::{
        actor::{
            Actor, ActorAggression, ActorBehavior, ActorBehaviorFamily, ActorBehaviorParams,
            ActorController, ActorControllerKind, ActorParams, ActorStats,
        },
        draw_buffer::{Bounds, BufferedDraw, DrawBuffer},
        item::{Item, ItemKind, ItemParams},
        light_source::LightSource,
        projectiles::{ProjectileKind, Projectiles},
        CameraController, ContinuousBeams, Credits, GameState, Hud, MapRenderer, PostProcessing,
    },
    noise_level::NoiseLevel,
    physics::{raycast, Collider, CollisionKind, PhysicsBody},
    player::{get_player_actor, LocalPlayer},
    render::{
        color_from_hex_string, draw_progress_bar, draw_text, draw_texture, use_default_material,
        use_material, HorizontalAlignment, Material, Sprite, SpriteAnimationParams,
        SpriteAnimationPlayer, Texture, VerticalAlignment, Viewport, COLOR_NONE,
    },
    resources::Resources,
    scene::{DrawStage, SceneBuilder, SceneBuilderFunc},
    versions::{check_version, get_toolkit_version, to_int_version},
};
