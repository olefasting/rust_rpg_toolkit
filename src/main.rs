#![feature(fn_traits)]
#![feature(drain_filter)]
#![feature(try_find)]

use macroquad::{
    color,
    experimental::{
        collections::storage,
        coroutines::start_coroutine,
        scene,
    },
    prelude::*,
};

use map::{
    Map,
};
use nodes::{
    Actor,
    ActorControllerKind,
    ActorParams,
    Camera,
    ContinuousBeams,
    DrawBuffer,
    GameState,
    Item,
    Projectiles,
    ItemParams,
    PostProcessing,
};
use physics::Collider;
use render::{
    draw_aligned_text,
    HorizontalAlignment,
};
use resources::Resources;
use crate::render::VerticalAlignment;

pub mod resources;
pub mod ability;
pub mod map;
pub mod nodes;
pub mod render;
pub mod input;
pub mod physics;
pub mod math;
pub mod gui;
pub mod json;
pub mod helpers;

const CRT_FRAGMENT_SHADER: &'static str = r#"#version 100
precision lowp float;
varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;
// https://www.shadertoy.com/view/XtlSD7
vec2 CRTCurveUV(vec2 uv)
{
    uv = uv * 2.0 - 1.0;
    vec2 offset = abs( uv.yx ) / vec2( 6.0, 4.0 );
    uv = uv + uv * offset * offset;
    uv = uv * 0.5 + 0.5;
    return uv;
}
void DrawVignette( inout vec3 color, vec2 uv )
{
    float vignette = uv.x * uv.y * ( 1.0 - uv.x ) * ( 1.0 - uv.y );
    vignette = clamp( pow( 16.0 * vignette, 0.3 ), 0.0, 1.0 );
    color *= vignette;
}
void DrawScanline( inout vec3 color, vec2 uv )
{
    float iTime = 0.1;
    float scanline 	= clamp( 0.95 + 0.05 * cos( 3.14 * ( uv.y + 0.008 * iTime ) * 240.0 * 1.0 ), 0.0, 1.0 );
    float grille 	= 0.85 + 0.15 * clamp( 1.5 * cos( 3.14 * uv.x * 640.0 * 1.0 ), 0.0, 1.0 );
    color *= scanline * grille * 1.2;
}
void main() {

    vec2 crtUV = CRTCurveUV(uv);

    vec3 res = texture2D(Texture, uv).rgb * color.rgb;

    if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0)
    {
        res = vec3(0.0, 0.0, 0.0);
    }
    DrawVignette(res, crtUV);
    DrawScanline(res, uv);
    gl_FragColor = vec4(res, 1.0);
}
"#;

const CRT_VERTEX_SHADER: &'static str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying lowp vec2 uv;
varying lowp vec4 color;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0 / 255.0;
    uv = texcoord;
}
";

pub fn generate_id() -> String {
    nanoid::nanoid!()
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
        storage::store(resources);
    });

    while load_resources.is_done() == false {
        clear_background(BLACK);
        draw_aligned_text(
            &format!("Loading resources"),
            screen_width() / 2.0,
            screen_height() / 2.0,
            HorizontalAlignment::Center,
            VerticalAlignment::Center,
            TextParams {
                font_size: 40,
                color: color::WHITE,
                ..Default::default()
            },
        );

        next_frame().await;
    }

    {
        let post_processing_material = load_material(
            CRT_VERTEX_SHADER,
            CRT_FRAGMENT_SHADER,
            Default::default(),
        ).unwrap();

        let player_id = generate_id();

        // let map = Map::load_tiled(
        //     "assets/maps/test_tiled_map.json",
        //     Some("assets/maps/map_01.json"),
        //     Some(&[
        //         ("barriers_2", map::MapCollisionKind::Barrier),
        //         ("barriers_1", map::MapCollisionKind::Barrier),
        //         ("solids_2", map::MapCollisionKind::Solid),
        //         ("solids_1", map::MapCollisionKind::Solid),
        //     ]),
        //     &[
        //         ("neo_zero_tiles", "../textures/neo_zero_tiles.png", "tiles"),
        //         ("neo_zero_props", "../textures/neo_zero_props.png", "props"),
        //         ("items", "../textures/items.png", "items"),
        //     ]).unwrap();

        let map = Map::load("assets/maps/map_01.json").unwrap();
        // let map = Map::load("assets/maps/test_capstone_map.json").unwrap();

        GameState::add_node(map, &player_id.clone());

        Camera::add_node();

        DrawBuffer::<Item>::add_node();

        Projectiles::add_node();
        ContinuousBeams::add_node();

        DrawBuffer::<Actor>::add_node();

        PostProcessing::add_node(post_processing_material);

        {
            let game_state = scene::find_node_by_type::<GameState>().unwrap();
            let resources = storage::get::<Resources>();
            if let Some(layer) = game_state.map.layers.get("spawn_points") {
                for object in &layer.objects {
                    if object.name == "player" {
                        let params = resources.actors.get("player").cloned().unwrap();
                        let mut player = Actor::new(
                            ActorControllerKind::LocalPlayer { player_id: player_id.clone() },
                            ActorParams {
                                name: "Abraxas".to_string(),
                                position: Some(object.position),
                                ..params
                            }
                        );
                        player.stats.recalculate_derived();
                        player.stats.restore_vitals();
                        scene::add_node(player);
                    } else if let Some(prototype_id) = object.properties.get("prototype_id") {
                        if let Some(params) = resources.actors.get(prototype_id).cloned() {
                            let mut actor = Actor::new(ActorControllerKind::Computer, ActorParams {
                                position: Some(object.position),
                                ..params
                            });
                            actor.stats.recalculate_derived();
                            actor.stats.restore_vitals();
                            scene::add_node(actor);
                        } else {
                            println!("actor prototype id '{}' not found!", prototype_id);
                        }
                    }
                }
            }

            if let Some(layer) = game_state.map.layers.get("items") {
                for object in &layer.objects {
                    if let Some(prototype_id) = object.properties.get("prototype_id") {
                        if let Some(params) = resources.items.get(prototype_id).cloned() {
                            Item::add_node(ItemParams {
                                position: Some(object.position),
                                ..params
                            });
                        } else {
                            println!("item prototype id '{}' not found!", prototype_id);
                        }
                    }
                }
            }
        }
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
