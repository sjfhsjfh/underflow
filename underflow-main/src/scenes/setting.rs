use comui::{
    component::Component,
    components::DataComponent,
    layout::{Layout, LayoutBuilder},
    scene::{NextScene, Scene},
    utils::Transform,
    window::Window,
};
use underflow_l10n::LANGS;

use crate::{
    components::{button::LabeledButton, data_bar::DataBar, single_choice::SingleChoice},
    config::{get_config_mut, sync_config},
    tl,
};

pub struct SettingScene {
    backbtn: LabeledButton,
    lang_bar: DataBar<String, SingleChoice>,
}

impl Default for SettingScene {
    fn default() -> Self {
        Self {
            backbtn: LabeledButton::back_btn(),
            lang_bar: DataBar::new(
                tl!("language").to_string(),
                SingleChoice::new(LANGS.iter().map(|lang| lang.to_string()).collect(), 0),
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

    fn before_render(&mut self, _: &Transform, _: &mut Window) {
        self.lang_bar.name.text = tl!("language").to_string();
    }

    fn after_render(&mut self, _: &Transform, _: &mut Window) {
        if self.lang_bar.data.updated() {
            get_config_mut().language = Some(self.lang_bar.data.get_data().clone());
            sync_config();
        }
    }
}

impl Scene for SettingScene {
    fn next_scene(&mut self) -> Option<NextScene> {
        if self.backbtn.triggered() {
            Some(NextScene::Pop)
        } else {
            None
        }
    }
}
