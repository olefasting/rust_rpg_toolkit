use crate::gui::*;

pub struct WindowBuilder {
    id: Id,
    size: Vec2,
    position: Vec2,
    title: Option<String>,
    is_static: bool,
}

impl WindowBuilder {
    pub fn new(id: Id, size: Vec2) -> Self {
        WindowBuilder {
            id,
            size,
            position: Vec2::ZERO,
            title: None,
            is_static: false,
        }
    }

    pub fn with_title(self, title: &str) -> Self {
        WindowBuilder {
            title: Some(title.to_string()),
            ..self
        }
    }

    pub fn with_pos(self, position: Vec2, is_static: bool) -> Self {
        WindowBuilder {
            position,
            is_static,
            ..self
        }
    }

    pub fn with_centered_pos(self, is_static: bool) -> Self {
        WindowBuilder {
            position: get_centered_on_screen(self.size),
            is_static,
            ..self
        }
    }

    pub fn build<F: FnOnce(&mut Ui)>(&self, ui: &mut Ui, f: F) -> bool {
        let window = widgets::Window::new(self.id, self.position, self.size)
            .movable(self.is_static)
            .titlebar(false);

        if let Some(title) = &self.title {
            return window.ui(ui, |ui| {
                let gui_skins = storage::get::<GuiSkins>();
                ui.push_skin(&gui_skins.window_title);
                ui.label(None, &title);
                ui.pop_skin();

                f(ui);
            })
        }

        window.ui(ui, f)
    }
}
