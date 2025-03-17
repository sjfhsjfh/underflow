use std::cell::RefCell;

use macroquad::{
    input::{
        KeyCode, MouseButton, Touch, TouchPhase, is_mouse_button_down, mouse_position,
        utils::{register_input_subscriber, repeat_all_miniquad_input},
    },
    math::vec2,
    miniquad::{self, EventHandler},
};
use once_cell::sync::Lazy;

static SUBSCRIBER_ID: Lazy<usize> = Lazy::new(register_input_subscriber);
thread_local! {
    pub(crate) static TOUCHES: RefCell<(Vec<Touch>, i32, u32)> = RefCell::default();
}

/// Handle mouse click & touch events
pub(crate) fn on_frame() {
    let mut handler = Handler(Vec::new(), 0, 0);
    repeat_all_miniquad_input(&mut handler, *SUBSCRIBER_ID);
    handler.finalize();
    TOUCHES.with(|it| {
        *it.borrow_mut() = (handler.0, handler.1, handler.2);
    });
}

struct Handler(Vec<Touch>, i32, u32);
impl Handler {
    fn finalize(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            self.0.push(Touch {
                id: button_to_id(MouseButton::Left),
                phase: TouchPhase::Moved,
                position: mouse_position().into(),
            });
        }
    }
}

fn button_to_id(button: MouseButton) -> u64 {
    u64::MAX
        - match button {
            MouseButton::Left => 0,
            MouseButton::Middle => 1,
            MouseButton::Right => 2,
            MouseButton::Unknown => 3,
        }
}

impl EventHandler for Handler {
    fn update(&mut self) {}
    fn draw(&mut self) {}
    fn touch_event(&mut self, phase: miniquad::TouchPhase, id: u64, x: f32, y: f32) {
        self.0.push(Touch {
            id,
            phase: phase.into(),
            position: vec2(x, y),
        });
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.0.push(Touch {
            id: button_to_id(button),
            phase: TouchPhase::Started,
            position: vec2(x, y),
        });
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.0.push(Touch {
            id: button_to_id(button),
            phase: TouchPhase::Ended,
            position: vec2(x, y),
        });
    }

    fn key_down_event(&mut self, _keycode: KeyCode, _keymods: miniquad::KeyMods, repeat: bool) {
        if !repeat {
            self.1 += 1;
            self.2 += 1;
        }
    }

    fn key_up_event(&mut self, _keycode: KeyCode, _keymods: miniquad::KeyMods) {
        self.1 -= 1;
    }
}
