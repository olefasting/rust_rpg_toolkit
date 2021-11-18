use rust_rpg_toolkit::prelude::*;

pub struct ExampleNode {
    pub position: Vec2,
    pub size: Vec2,
    pub sprite: SpriteAnimationPlayer,
}

impl ExampleNode {
    pub fn new(position: Vec2, size: Vec2, sprite_params: SpriteAnimationParams) -> Self {
        let sprite = SpriteAnimationPlayer::new(sprite_params);

        ExampleNode {
            position,
            size,
            sprite,
        }
    }

    pub fn add_node(
        position: Vec2,
        size: Vec2,
        sprite_params: SpriteAnimationParams,
    ) -> Handle<Self> {
        let node = Self::new(position, size, sprite_params);
        scene::add_node(node)
    }
}

impl BufferedDraw for ExampleNode {
    fn buffered_draw(&mut self) {
        // Draw node here, in stead of in Node::draw
        self.sprite.draw(self.position, 0.0);
    }

    fn get_z_index(&self) -> f32 {
        // This is used to determine the order of draw calls, within
        // the DrawBuffer. For normal sized actors, you can use y position
        self.position.y
    }

    fn get_bounds(&self) -> Bounds {
        // This is used when frustum culling (if bounds are within viewport,
        // it will be drawn.
        let rect = Rect::new(self.position.x, self.position.y, self.size.x, self.size.y);
        Bounds::Rectangle(rect)
    }
}

impl Node for ExampleNode {
    fn ready(node: RefMut<Self>) {
        // Add the node to the appropriate DrawBuffer. If you have multiple DrawBuffers for one
        // type, for some reason, you'll have to make this a bit more elaborate.
        let mut draw_buffer = scene::find_node_by_type::<DrawBuffer<Self>>().unwrap();
        draw_buffer.buffered.push(node.handle());
    }

    fn update(mut node: RefMut<Self>) {
        // Update node state
        node.sprite.update();
    }

    fn fixed_update(_: RefMut<Self>) {
        // Update physics state
    }

    fn draw(_: RefMut<Self>) {
        // Don't use this as it will be called by Macroquad, in addition to your `buffered_draw`
        // implementation, only according to the order the node was added to the scene tree
    }
}
