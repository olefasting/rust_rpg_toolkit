use macroquad::math::Vec2;

#[derive(Clone)]
pub struct ActorController {
    pub directions: Vec2,
    pub is_player: bool,
}

impl ActorController {
    pub fn new(is_player: bool) -> Self {
        ActorController {
            directions: Vec2::ZERO,
            is_player,
        }
    }

    pub fn clear(&mut self) {
        self.directions = Vec2::ZERO;
    }

    pub fn set_direction(&mut self, directions: Vec2) {
        self.directions = directions;
    }
}
