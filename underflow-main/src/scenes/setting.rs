use comui::{
    component::Component,
    components::{DataComponent, label::Align},
    layout::{Layout, LayoutBuilder},
    scene::{NextScene, Scene},
    utils::Transform,
    window::Window,
};
use underflow_l10n::{LANG_NAMES, LANGS};

use crate::{
    colors,
    components::{button::LabeledButton, data_bar::DataBar, single_choice::SingleChoice},
    config::{get_config, get_config_mut, sync_config},
    tl,
};

pub struct SettingScene {
    backbtn: LabeledButton,
    lang_bar: DataBar<String, SingleChoice>,
}

impl SettingScene {
    const DATA_FONT_SIZE: f32 = 48.0;
}

impl Default for SettingScene {
    fn default() -> Self {
        Self {
            backbtn: LabeledButton::back_btn(),
            lang_bar: DataBar::new(
                tl!("language").to_string(),
                SingleChoice::new(
                    LANG_NAMES.iter().map(|lang| lang.to_string()).collect(),
                    get_config()
                        .language
                        .as_ref()
                        .map(|lang| LANGS.iter().position(|l| lang.as_str() == *l).unwrap_or(0))
                        .unwrap_or(0),
                    |l| {
                        l.with_align(Align::Center)
                            .with_font_size(Self::DATA_FONT_SIZE)
                            .with_line_height(Self::DATA_FONT_SIZE)
                            .with_color(colors::BLACK)
                            .with_texture_align((0.5, 0.6))
                    },
                ),
            ),
        }
    }
}

impl Layout for SettingScene {
    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        LayoutBuilder::new()
            .at_rect(super::BACK_BTN_RECT, &mut self.backbtn)
            .at_rect((0.0, 0.0, 0.8, 0.15), &mut self.lang_bar)
            .build()
    }

    fn before_render(&mut self, _: &Transform, _: &mut Window) {
        self.lang_bar.name.text = tl!("language").to_string();
    }

    fn after_render(&mut self, _: &Transform, _: &mut Window) {
        if self.lang_bar.data.updated() {
            let lang_idx = LANG_NAMES
                .iter()
                .position(|lang| *lang == self.lang_bar.data.get_data())
                .unwrap();
            get_config_mut().language = Some(LANGS[lang_idx].to_string());
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
