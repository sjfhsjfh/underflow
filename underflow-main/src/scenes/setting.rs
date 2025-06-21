use comui::{
    component::Component,
    layout::{Layout, LayoutBuilder},
    scene::{NextScene, Scene},
    utils::Transform,
};
use underflow_l10n::LANGS;

use crate::{
    components::{button::RoundedButton, data_bar::DataBar, popup_select::PopUpSelect},
    tl,
};

pub struct SettingScene {
    backbtn: RoundedButton,
    lang_bar: DataBar<String, PopUpSelect<String>>,
}

impl Default for SettingScene {
    fn default() -> Self {
        Self {
            backbtn: RoundedButton::back_btn(),
            lang_bar: DataBar::new(
                tl!("language").to_string(),
                PopUpSelect {
                    selected: 0,
                    options: LANGS.iter().map(|lang| lang.to_string()).collect(),
                },
            ),
        }
    }
}

impl Layout for SettingScene {
    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        LayoutBuilder::new()
            .at_rect(
                super::BACK_BTN_RECT,
                &mut self.backbtn as &mut dyn Component,
            )
            .at_rect(
                (0.0, 0.0, 0.8, 0.15),
                &mut self.lang_bar as &mut dyn Component,
            )
            .build()
    }

    fn before_render(&mut self, _: &Transform, _: &mut comui::window::Window) {
        self.lang_bar.name.text = tl!("language").to_string();
    }
}

impl Scene for SettingScene {
    fn next_scene(&mut self) -> Option<NextScene> {
        if self.backbtn.inner.triggered {
            Some(NextScene::Pop)
        } else {
            None
        }
    }
}
