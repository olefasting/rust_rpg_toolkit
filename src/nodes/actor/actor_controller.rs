use macroquad::math::Vec2;

#[derive(Clone)]
pub struct ActorController {
    pub destination: Option<Vec2>,
    pub direction: Vec2,
    pub player_id: Option<u32>,
}

impl ActorController {
    pub fn new(player_id: Option<u32>) -> Self {
        ActorController {
            destination: None,
            direction: Vec2::ZERO,
            player_id,
        }
    }

    #[allow(dead_code)]
    pub fn clear_all(&mut self) {
        self.direction = Vec2::ZERO;
        self.destination = None;
    }
}
