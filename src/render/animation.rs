use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteAnimationParams {
    #[serde(with = "json::def_vec2")]
    pub offset: Vec2,
    pub texture_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal_map_id: Option<String>,
    #[serde(with = "json::def_vec2")]
    pub tile_size: Vec2,
    #[serde(with = "json::vec_animation")]
    pub animations: Vec<Animation>,
    #[serde(skip)]
    pub should_play: bool,
}

impl Default for SpriteAnimationParams {
    fn default() -> Self {
        SpriteAnimationParams {
            offset: vec2(-8.0, -8.0),
            texture_id: Resources::WHITE_TEXTURE_ID.to_string(),
            normal_map_id: None,
            tile_size: vec2(16.0, 16.0),
            animations: vec![Animation {
                name: "idle".to_string(),
                row: 0,
                frames: 1,
                fps: 8,
            }],
            should_play: false,
        }
    }
}

#[derive(Clone)]
pub struct SpriteAnimationPlayer {
    pub offset: Vec2,
    pub rotation: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    texture_id: String,
    normal_map_id: Option<String>,
    tile_size: Vec2,
    animations: Vec<Animation>,
    animated_sprite: AnimatedSprite,
}

impl SpriteAnimationPlayer {
    pub fn new(params: SpriteAnimationParams) -> Self {
        let animated_sprite = AnimatedSprite::new(
            params.tile_size.x as u32,
            params.tile_size.y as u32,
            &params.animations,
            params.should_play,
        );

        SpriteAnimationPlayer {
            offset: params.offset,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            texture_id: params.texture_id,
            normal_map_id: params.normal_map_id,
            tile_size: params.tile_size,
            animations: params.animations,
            animated_sprite,
        }
    }

    pub fn is_playing(&self) -> bool {
        self.animated_sprite.playing
    }

    pub fn set_animation(&mut self, id: usize) {
        self.animated_sprite.set_animation(id);
    }

    pub fn start_animation(&mut self, id: usize) {
        self.set_animation(id);
        self.play();
    }

    pub fn restart_animation(&mut self) {
        self.animated_sprite.set_frame(0);
    }

    pub fn set_frame(&mut self, frame: u32) {
        self.animated_sprite.set_frame(frame);
    }

    pub fn play(&mut self) {
        self.animated_sprite.playing = true;
    }

    pub fn stop(&mut self) {
        self.animated_sprite.playing = false;
    }

    pub fn update(&mut self) {
        self.animated_sprite.update();
    }

    pub fn draw(&mut self, position: Vec2, rotation: f32) {
        let resources = storage::get::<Resources>();
        let texture = resources.textures.get(&self.texture_id).unwrap();
        draw_texture(
            texture,
            position + self.offset,
            None,
            DrawTextureParams {
                source: Some(self.animated_sprite.frame().source_rect),
                dest_size: Some(self.animated_sprite.frame().dest_size),
                flip_x: self.flip_x,
                flip_y: self.flip_y,
                rotation: self.rotation + rotation,
                ..Default::default()
            },
        );
    }
}

impl From<SpriteAnimationPlayer> for SpriteAnimationParams {
    fn from(player: SpriteAnimationPlayer) -> SpriteAnimationParams {
        SpriteAnimationParams {
            offset: player.offset,
            texture_id: player.texture_id,
            normal_map_id: player.normal_map_id,
            tile_size: player.tile_size,
            animations: player.animations,
            should_play: player.animated_sprite.playing,
        }
    }
}
