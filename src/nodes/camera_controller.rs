use crate::prelude::*;

#[derive(Default)]
pub struct CameraController {
    pub position: Vec2,
    pub rotation: f32,
    pub is_following: bool,
    scale: f32,
    render_target: Option<RenderTarget>,
}

impl CameraController {
    const FOLLOW_THRESHOLD_FRACTION: f32 = 0.4;
    const FOLLOW_END_AT_DISTANCE: f32 = 20.0;
    const FOLLOW_LERP_FRACTION: f32 = 0.03;

    const DEFAULT_SCALE: f32 = 2.0;

    pub fn new() -> Self {
        let scale = Self::DEFAULT_SCALE;

        let mut res = CameraController {
            position: Vec2::ZERO,
            rotation: 0.0,
            scale,
            is_following: false,
            render_target: None,
        };

        res.create_render_target();

        res
    }

    pub fn add_node() -> Handle<Self> {
        scene::add_node(CameraController::new())
    }

    pub fn get_viewport(&self) -> Viewport {
        let size = vec2(
            get_screen_width() / self.scale,
            get_screen_height() / self.scale,
        );
        let position = self.position - size / 2.0;
        Viewport {
            position,
            size,
            scale: self.scale,
        }
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        self.create_render_target();
    }

    pub fn get_render_target(&self) -> Option<&RenderTarget> {
        self.render_target.as_ref()
    }

    fn get_camera(&self) -> Camera2D {
        Camera2D {
            offset: vec2(0.0, 0.0),
            target: vec2(self.position.x.round(), self.position.y.round()),
            zoom: vec2(
                self.scale / get_screen_width(),
                self.scale / get_screen_height(),
            ) * 2.0,
            rotation: self.rotation,
            render_target: self.render_target,
            ..Camera2D::default()
        }
    }

    fn create_render_target(&mut self) {
        let res = new_render_target(
            (get_screen_width() / self.scale) as u32,
            (get_screen_height() / self.scale) as u32,
        );

        res.texture.set_filter(FilterMode::Nearest);

        self.render_target = Some(res);
    }
}

impl Node for CameraController {
    fn ready(node: RefMut<Self>) {
        storage::store(node.get_viewport());
    }

    fn fixed_update(mut node: RefMut<Self>) {
        let viewport = node.get_viewport();

        if let Some(actor) = get_player_actor() {
            let bounds = {
                let size = viewport.size * Self::FOLLOW_THRESHOLD_FRACTION;
                let center = viewport.get_center();
                Rect::new(
                    center.x - size.x / 2.0,
                    center.y - size.y / 2.0,
                    size.x,
                    size.y,
                )
            };

            if node.is_following || !bounds.contains(actor.body.position) {
                let distance = actor.body.position.sub(node.position);
                if distance.length() > Self::FOLLOW_END_AT_DISTANCE {
                    node.is_following = true;
                    node.position += distance * Self::FOLLOW_LERP_FRACTION;
                } else {
                    node.is_following = false;
                }
            }
        }

        storage::store(viewport);
    }

    fn draw(node: RefMut<Self>)
    where
        Self: Sized,
    {
        scene::set_camera(0, Some(node.get_camera()));
    }
}
