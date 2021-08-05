use macroquad::math::Vec2;

#[derive(Clone)]
pub struct ActorController {
    pub destination: Option<Vec2>,
    pub direction: Vec2,
    pub is_player: bool,
}

impl ActorController {
    pub fn new(is_player: bool) -> Self {
        ActorController {
            destination: None,
            direction: Vec2::ZERO,
            is_player,
        }
    }

    pub fn clear_all(&mut self) {
        self.direction = Vec2::ZERO;
        self.destination = None;
    }
}
