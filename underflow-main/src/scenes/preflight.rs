use comui::{
    component::Component,
    layout::{Layout, LayoutBuilder},
    scene::{NextScene, Scene},
    utils::Transform,
};

use crate::components::button::RoundedButton;

pub struct PreflightScene {
    back_btn: RoundedButton,
}

impl Default for PreflightScene {
    fn default() -> Self {
        Self {
            back_btn: RoundedButton::back_btn(),
        }
    }
}

impl Layout for PreflightScene {
    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        LayoutBuilder::new()
            .at_rect(
                super::BACK_BTN_RECT,
                &mut self.back_btn as &mut dyn Component,
            )
            .build()
    }
}

impl Scene for PreflightScene {
    fn next_scene(&mut self) -> Option<NextScene> {
        if self.back_btn.inner.triggered {
            Some(NextScene::Pop)
        } else {
            None
        }
    }
}
