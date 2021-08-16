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
    prelude::*,
};

use crate::{
    get_global,
    Resources,
    render::Viewport,
};
use crate::physics::Collider;
use crate::math::Circle;

pub trait BufferedDraw : Node {
    fn buffered_draw(&mut self);

    fn get_z_index(&self) -> f32;

    fn is_in_view(&self, _view_rect: &Rect) -> bool {
        true
    }
}

pub trait CulledPosition: BufferedDraw {
    fn get_position(&self) -> Vec2;

    fn is_in_view(&self, view_rect: &Rect) -> bool {
        view_rect.contains(self.get_position())
    }
}

pub trait CulledRect: BufferedDraw {
    fn get_bounds_as_rect(&self) -> Rect;

    fn is_in_view(&self, view_rect: &Rect) -> bool {
        view_rect.overlaps(&self.get_bounds_as_rect())
    }
}

pub trait CulledCircle: BufferedDraw {
    fn get_bounds_as_circle(&self) -> Circle;

    fn is_in_view(&self, view_rect: &Rect) -> bool {
        self.get_bounds_as_circle().overlaps_rect(&view_rect)
    }
}

pub trait CulledCollider: BufferedDraw {
    fn get_bounds_as_offset_collider(&self) -> Collider;

    fn is_in_view(&self, view_rect: &Rect) -> bool {
        self.get_bounds_as_offset_collider().overlaps_rect(&view_rect)
    }
}

pub struct DrawBuffer<T: 'static + BufferedDraw> {
    pub nodes: Vec<Handle<T>>,
}

impl<T: 'static + BufferedDraw> DrawBuffer<T> {
    pub fn new() -> Self {
        DrawBuffer {
            nodes: Vec::new(),
        }
    }

    pub fn add_node() -> Handle<Self> {
        scene::add_node(Self::new())
    }
}

impl<T: 'static + BufferedDraw> Node for DrawBuffer<T> {
    fn draw(mut node: RefMut<Self>) {
        let view_rect = get_global::<Viewport>().to_rect();
        node.nodes.retain(|handle| if let Some(buffered) = scene::try_get_node(*handle) {
            buffered.is_in_view(&view_rect)
        } else {
            false
        });

        node.nodes.sort_by(|a, b| {
            if let Some(a) = scene::try_get_node(*a) {
                if let Some(b) = scene::try_get_node(*b) {
                    return a.get_z_index().partial_cmp(&b.get_z_index()).unwrap();
                }
            }
            Ordering::Equal
        });

        for handle in &node.nodes {
            let mut buffered = scene::get_node(*handle);
            buffered.buffered_draw();
        }
    }
}
