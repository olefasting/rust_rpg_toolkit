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
};

use super::{
    Actor,
    GameState,
};

pub struct Hud {
}

impl Hud {
    pub fn new() -> Self {
        Hud {
        }
    }

    pub fn add_node() -> Handle<Self> {
        scene::add_node(Self::new())
    }
}

impl Node for Hud {
    fn draw(_node: RefMut<Self>) {
        let game_state = scene::find_node_by_type::<GameState>().unwrap();

        push_camera_state();
        set_default_camera();

        if game_state.in_debug_mode {
            push_camera_state();
            set_default_camera();
            draw_aligned_text(
                "DEBUG MODE",
                screen_width() / 2.0,
                50.0,
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
                50.0,
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
                    let len = player.active_missions.len();
                    if len > 0 {
                        draw_aligned_text(
                            "Active missions:",
                            screen_width() - 50.0,
                            250.0,
                            HorizontalAlignment::Right,
                            VerticalAlignment::Center,
                            Default::default(),
                        );
                    }

                    for i in 0..len {
                        let mission = player.active_missions.get(i).unwrap();
                        draw_aligned_text(
                            &mission.title,
                            screen_width() - 50.0,
                            300.0 + i as f32 * 50.0,
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
                            screen_width() - 50.0,
                            500.0,
                            HorizontalAlignment::Right,
                            VerticalAlignment::Center,
                            Default::default(),
                        );
                    }

                    for i in 0..len {
                        let mission = player.completed_missions.get(i).unwrap();
                        draw_aligned_text(
                            &mission.title,
                            screen_width() - 50.0,
                            550.0 + i as f32 * 50.0,
                            HorizontalAlignment::Right,
                            VerticalAlignment::Center,
                            Default::default(),
                        )
                    }
                }

                pop_camera_state();


                let height = Actor::HEALTH_BAR_HEIGHT * viewport.scale;
                let (position, offset_y, alignment, length, height, border) =
                    (vec2(10.0, 10.0), height / 2.0, HorizontalAlignment::Left, Actor::HEALTH_BAR_LENGTH * viewport.scale, height, viewport.scale);

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
