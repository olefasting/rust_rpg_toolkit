#![feature(fn_traits)]
#![feature(drain_filter)]

use macroquad::{
    color,
    experimental::{
        coroutines::start_coroutine,
        scene,
    },
    prelude::*,
};
use macroquad::prelude::animation::Animation;

pub use globals::{
    get_global,
    set_global,
    try_get_global,
};
use globals::LocalPlayer;
pub use input::get_mouse_position;
pub use map::{
    Map,
};
use nodes::{
    Actor,
    ActorControllerKind,
    ActorParams,
    Camera,
    GameState,
    Projectiles,
    Item,
    ItemParams,
};
use physics::Collider;
use render::{
    HorizontalAlignment,
    SpriteAnimationParams,
    SpriteParams,
};
pub use render::text::draw_aligned_text;
pub use resources::Resources;

use crate::nodes::actor::{ActorDrawBuffer, ActorStats};
use crate::nodes::item::ItemDrawBuffer;

pub use actions::{
    ActionFunc,
    ActionFuncs
};
use crate::actions::ActionParams;

mod resources;
mod globals;
mod map;
mod actions;

pub mod nodes;
pub mod render;
pub mod input;
pub mod physics;
pub mod math;
pub mod gui;

pub fn generate_id() -> String {
    nanoid::nanoid!()
}

fn generic_ranged_weapon() -> ItemParams {
    ItemParams {
        kind: Item::ONE_HANDED_WEAPON_KIND.to_string(),
        name: "Test Ranged Weapon".to_string(),
        description: "Test Ranged Weapon description".to_string(),
        weight: 10.0,
        action_params: ActionParams {
            action_func_id: ActionFuncs::PROJECTILE_ACTION_ID.to_string(),
            action_kind: ActionParams::PRIMARY_ABILITY.to_string(),
            cooldown: 0.0025,
            stamina_cost: 10.0,
            speed: 15.0,
            spread: 10.0,
            range: 15.0,
            damage: 10.0,
            ..Default::default()
        },
        sprite_params: SpriteParams {
            texture_id: Resources::ITEMS_TEXTURE_ID.to_string(),
            texture_coords: uvec2(0, 3),
            tile_size: vec2(16.0, 16.0),
            offset: vec2(-8.0, -8.0),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn generic_misc_item() -> ItemParams {
    ItemParams {
        kind: Item::MISC_KIND.to_string(),
        name: "Test Trinket".to_string(),
        description: "Test Trinket description".to_string(),
        weight: 1.0,
        action_params: ActionParams {
            action_func_id: ActionFuncs::MAGIC_SPHERE_ACTION_ID.to_string(),
            action_kind: ActionParams::SECONDARY_ABILITY.to_string(),
            cooldown: 0.75,
            stamina_cost: 10.0,
            energy_cost: 100.0,
            speed: 1.0,
            spread: 0.0,
            range: 2.0,
            damage: 150.0,
            ..Default::default()
        },
        sprite_params: SpriteParams {
            texture_id: Resources::ITEMS_TEXTURE_ID.to_string(),
            texture_coords: uvec2(3, 3),
            tile_size: vec2(16.0, 16.0),
            offset: vec2(-8.0, -8.0),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn generic_actor(name: &str, position: Vec2, skin_id: u32, factions: &[String], player_id: Option<u32>) -> ActorParams {
    ActorParams {
        name: name.to_string(),
        factions: factions.to_vec(),
        position,
        stats: ActorStats::new(
            8,
            8,
            8,
            8,
            8,
            8,
            8,
        ),
        inventory: vec!(
            generic_ranged_weapon(),
            generic_ranged_weapon(),
            generic_ranged_weapon(),
            generic_misc_item(),
            generic_misc_item(),
            generic_misc_item(),
        ),
        collider: Some(Collider::circle(0.0, 8.0, 8.0)),
        controller_kind: match player_id {
            Some(id) => ActorControllerKind::Player { id },
            None => ActorControllerKind::Computer,
        },
        sprite_animation_params: SpriteAnimationParams {
            texture_id: Resources::CHARACTERS_TEXTURE_ID.to_string(),
            tile_size: vec2(32.0, 32.0),
            offset: vec2(-16.0, -16.0),
            animations: vec!(
                Animation {
                    name: "down".to_string(),
                    row: skin_id * 3,
                    frames: 3,
                    fps: 8,
                },
                Animation {
                    name: "up".to_string(),
                    row: skin_id * 3 + 1,
                    frames: 3,
                    fps: 8,
                },
                Animation {
                    name: "right".to_string(),
                    row: skin_id * 3 + 2,
                    frames: 3,
                    fps: 8,
                }
            ),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Capstone".to_owned(),
        high_dpi: false,
        window_width: 1920,
        window_height: 180,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let load_resources = start_coroutine(async move {
        let resources = Resources::new().await.unwrap();
        set_global(resources);

        let actions = ActionFuncs::new().await;
        set_global(actions);
    });

    while load_resources.is_done() == false {
        clear_background(BLACK);
        draw_aligned_text(
            &format!("Loading resources"),
            screen_width() / 2.0,
            screen_height() / 2.0,
            HorizontalAlignment::Center,
            TextParams {
                font_size: 40,
                color: color::WHITE,
                ..Default::default()
            }
        );

        next_frame().await;
    }

    {
        set_global(LocalPlayer {
            id: 0,
        });

        // TODO: Move to resources
        let map = Map::new(uvec2(16, 16), "assets/maps/map_01.json").await;
        GameState::add_node(map);

        Camera::add_node(vec2(100.0, 100.0));

        ItemDrawBuffer::add_node();

        Projectiles::add_node();

        Actor::add_node(generic_actor(
            "Player Actor",
            vec2(100.0, 100.0),
            0,
            &["player_faction".to_string()],
            Some(0),
        ));

        Actor::add_node(generic_actor(
            "Friendly Actor",
            vec2(100.0, 200.0),
            2,
            &["player_faction".to_string()],
            None,
        ));

        Actor::add_node(generic_actor(
            "Enemy Actor",
            vec2(250.0, 250.0),
            1,
            &[],
            None,
        ));

        ActorDrawBuffer::add_node();
    }

    loop {
        {
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            if game_state.should_quit {
                break;
            }
        }

        gui::draw_gui();

        next_frame().await;
    }

    scene::clear();
}
