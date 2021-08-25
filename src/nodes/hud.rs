use macroquad::{
    experimental::{
        scene::{
            Node,
            RefMut,
            Handle,
        },
        collections::storage,
    },
    color,
    prelude::*,
};

use crate::{
    render::{
        draw_progress_bar,
        draw_aligned_text,
        HorizontalAlignment,
        VerticalAlignment,
        Viewport,
    },
    Config,
};

use super::{
    Actor,
    GameState,
};
use crate::resources::Resources;
use crate::math::deg_to_rad;

pub struct Hud {}

impl Hud {
    pub fn new() -> Self {
        Hud {}
    }

    pub fn add_node() -> Handle<Self> {
        scene::add_node(Self::new())
    }
}

impl Node for Hud {
    fn draw(_node: RefMut<Self>) {
        let config = storage::get::<Config>();
        let game_state = scene::find_node_by_type::<GameState>().unwrap();

        push_camera_state();
        set_default_camera();

        if game_state.in_debug_mode {
            push_camera_state();
            set_default_camera();
            draw_aligned_text(
                "DEBUG MODE",
                screen_width() / 2.0,
                50.0 * config.gui_scale,
                HorizontalAlignment::Center,
                VerticalAlignment::Top,
                TextParams {
                    color: color::RED,
                    font_size: 36,
                    ..Default::default()
                },
            );
            draw_aligned_text(
                &format!("fps: {}", get_fps()),
                screen_width() - 50.0,
                50.0 * config.gui_scale,
                HorizontalAlignment::Right,
                VerticalAlignment::Top,
                Default::default(),
            );
            pop_camera_state();
        }

        {
            if let Some(player) = Actor::find_by_player_id(&game_state.local_player_id) {
                let viewport = storage::get::<Viewport>();

                push_camera_state();
                set_default_camera();

                {
                    let frustum = viewport.get_frustum();
                    let resources = storage::get::<Resources>();
                    let texture = resources.textures.get("mission_marker").unwrap();
                    for mission in &player.active_missions {
                        if let Some(marker) = mission.marker.clone() {
                            if let Some(position) = marker.get_position() {
                                if frustum.contains(position) {
                                    let position = viewport.to_screen_space(position);
                                    let color = marker.get_color();
                                    let rotation = 0.0;
                                    // let screen_height = screen_height();
                                    // let screen_width = screen_width();
                                    // if position.x > screen_width - 16.0 {
                                    //     position.x = screen_width - 16.0;
                                    //     rotation = deg_to_rad(90.0);
                                    // } else if position.x < 16.0 {
                                    //     position.x = 16.0;
                                    //     rotation = deg_to_rad(-90.0);
                                    // } else {
                                    //     position.x -= 16.0;
                                    // }
                                    // if position.y > screen_height - 16.0 {
                                    //     position.y = screen_height - 16.0;
                                    // } else if position.y < 16.0 {
                                    //     position.y = 16.0;
                                    //     rotation = deg_to_rad(180.0);
                                    // }
                                    draw_texture_ex(
                                        texture.clone(),
                                        position.x,
                                        position.y,
                                        color,
                                        DrawTextureParams {
                                            rotation,
                                            ..Default::default()
                                        },
                                    )
                                }
                            }
                        }
                        for (objective, is_completed) in mission.objectives.clone() {
                            if is_completed == false {
                                if let Some(position) = objective.get_marker_position() {
                                    if frustum.contains(position) {
                                        let position = viewport.to_screen_space(position);
                                        let color = objective.get_marker_color().unwrap_or(color::YELLOW);
                                        let rotation = 0.0;
                                        // let screen_height = screen_height();
                                        // let screen_width = screen_width();
                                        // if position.x > screen_width - 16.0 {
                                        //     position.x = screen_width - 16.0;
                                        //     rotation = deg_to_rad(90.0);
                                        // } else if position.x < 16.0 {
                                        //     position.x = 16.0;
                                        //     rotation = deg_to_rad(-90.0);
                                        // } else {
                                        //     position.x -= 16.0;
                                        // }
                                        // if position.y > screen_height - 16.0 {
                                        //     position.y = screen_height - 16.0;
                                        // } else if position.y < 16.0 {
                                        //     position.y = 16.0;
                                        //     rotation = deg_to_rad(180.0);
                                        // }
                                        draw_texture_ex(
                                            texture.clone(),
                                            position.x,
                                            position.y,
                                            color,
                                            DrawTextureParams {
                                                rotation,
                                                ..Default::default()
                                            },
                                        )
                                    }
                                }
                            }
                        }
                    }
                }
                {
                    let len = player.active_missions.len();
                    if len > 0 {
                        draw_aligned_text(
                            "Active missions:",
                            screen_width() - 50.0 * config.gui_scale,
                            250.0 * config.gui_scale,
                            HorizontalAlignment::Right,
                            VerticalAlignment::Center,
                            Default::default(),
                        );
                    }

                    for i in 0..len {
                        let mission = player.active_missions.get(i).unwrap();
                        draw_aligned_text(
                            &mission.title,
                            screen_width() - 50.0 * config.gui_scale,
                            (300.0 * config.gui_scale) + i as f32 * (50.0 * config.gui_scale),
                            HorizontalAlignment::Right,
                            VerticalAlignment::Center,
                            Default::default(),
                        )
                    }
                }
                {
                    let len = player.completed_missions.len();
                    if len > 0 {
                        draw_aligned_text(
                            "Completed missions:",
                            screen_width() - 50.0 * config.gui_scale,
                            400.0 * config.gui_scale,
                            HorizontalAlignment::Right,
                            VerticalAlignment::Center,
                            Default::default(),
                        );
                    }

                    for i in 0..len {
                        let mission = player.completed_missions.get(i).unwrap();
                        draw_aligned_text(
                            &mission.title,
                            screen_width() - 50.0 * config.gui_scale,
                            450.0 * config.gui_scale + i as f32 * (50.0 * config.gui_scale),
                            HorizontalAlignment::Right,
                            VerticalAlignment::Center,
                            Default::default(),
                        )
                    }
                }

                pop_camera_state();


                let height = (Actor::HEALTH_BAR_HEIGHT * viewport.scale) * config.gui_scale;
                let (position, offset_y, alignment, length, height, border) =
                    (vec2(10.0 * config.gui_scale, 10.0 * config.gui_scale), height / 2.0, HorizontalAlignment::Left, (Actor::HEALTH_BAR_LENGTH * viewport.scale) * config.gui_scale, height, viewport.scale);

                draw_progress_bar(
                    player.stats.current_health,
                    player.stats.max_health,
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
                draw_progress_bar(
                    player.stats.current_stamina,
                    player.stats.max_stamina,
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
                    player.stats.current_energy,
                    player.stats.max_energy,
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
            }
        }

        pop_camera_state();
    }
}
