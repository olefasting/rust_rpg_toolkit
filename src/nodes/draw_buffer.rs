use std::{
    cmp::Ordering,
};

use macroquad::{
    experimental::{
        scene::{
            Node,
            Handle,
            RefMut,
        },
    },
    color,
    prelude::*,
};

use crate::{Camera, get_global, Resources, render::Viewport, draw_aligned_text};
use crate::physics::Collider;
use crate::math::Circle;
use crate::render::HorizontalAlignment;
use crate::nodes::Actor;

pub enum Bounds {
    Point(Vec2),
    Rectangle(Rect),
    Circle(Circle),
    Collider(Collider),
}

pub trait BufferedDraw: Node {
    fn buffered_draw(&mut self);

    fn get_z_index(&self) -> f32;

    fn get_bounds(&self) -> Bounds;

    fn is_in_frustum(&self, frustum: &Rect) -> bool {
        match self.get_bounds() {
            Bounds::Point(vec) => frustum.contains(vec),
            Bounds::Rectangle(rect) => frustum.overlaps(&rect),
            Bounds::Circle(circle) => circle.overlaps_rect(&frustum),
            Bounds::Collider(collider) => collider.overlaps_rect(&frustum),
        }
    }
}

pub struct DrawBuffer<T: 'static + BufferedDraw> {
    pub buffered: Vec<Handle<T>>,
}

impl<T: 'static + BufferedDraw> DrawBuffer<T> {
    pub fn new() -> Self {
        DrawBuffer {
            buffered: Vec::new(),
        }
    }

    pub fn add_node() -> Handle<Self> {
        scene::add_node(Self::new())
    }
}

impl<T: 'static + BufferedDraw> Node for DrawBuffer<T> {
    fn draw(mut node: RefMut<Self>) {
        let viewport = get_global::<Viewport>();
        let frustum = viewport.get_frustum_rect();
        node.buffered.retain(|handle| if let Some(buffered) = scene::try_get_node(*handle) {
            buffered.is_in_frustum(&frustum)
        } else {
            false
        });

        node.buffered.sort_by(|a, b| {
            if let Some(a) = scene::try_get_node(*a) {
                if let Some(b) = scene::try_get_node(*b) {
                    return a.get_z_index().partial_cmp(&b.get_z_index()).unwrap();
                }
            }
            Ordering::Equal
        });

        for handle in &node.buffered {
            let mut buffered = scene::get_node(*handle);
            buffered.buffered_draw();
        }
    }
}
