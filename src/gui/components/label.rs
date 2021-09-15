use crate::gui::*;

pub struct Label {
    position: Option<Vec2>,
    body: String,
}

impl Label {
    pub fn new<P: Into<Option<Vec2>>>(position: P, body: &str) -> Self {
        Label {
            position: position.into(),
            body: body.to_string(),
        }
    }
}

impl GuiComponent for Label {
    fn draw(&mut self, ui: &mut Ui) {
        ui.label(self.position,&self.body);
    }
}