use comui::input::subscriber_id;
use macroquad::{
    input::{
        MouseButton, Touch, TouchPhase, is_mouse_button_down, mouse_position,
        utils::repeat_all_miniquad_input,
    },
    math::vec2,
    miniquad::EventHandler,
    window::screen_dpi_scale,
};

fn button_to_id(button: MouseButton) -> u64 {
    u64::MAX
        - match button {
            MouseButton::Left => 0,
            MouseButton::Middle => 1,
            MouseButton::Right => 2,
            MouseButton::Unknown => 3,
        }
}

#[derive(Default)]
pub struct InputHandler {
    pub touches: Vec<Touch>,
}

impl EventHandler for InputHandler {
    fn draw(&mut self) {}

    fn update(&mut self) {
        repeat_all_miniquad_input(self, subscriber_id());
        if is_mouse_button_down(MouseButton::Left) {
            self.touches.push(Touch {
                id: button_to_id(MouseButton::Left),
                phase: TouchPhase::Moved,
                position: mouse_position().into(),
            });
        }
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.touches.push(Touch {
            id: button_to_id(button),
            phase: TouchPhase::Started,
            position: vec2(x, y) / screen_dpi_scale(),
        });
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.touches.push(Touch {
            id: button_to_id(button),
            phase: TouchPhase::Ended,
            position: vec2(x, y) / screen_dpi_scale(),
        });
    }
}
