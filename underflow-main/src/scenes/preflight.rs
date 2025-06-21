use comui::{
    component::Component,
    components::label::Align,
    layout::{Layout, LayoutBuilder},
    scene::{NextScene, Scene},
    utils::Transform,
};
use macroquad::color::Color;

use crate::{
    colors,
    components::{button::LabeledButton, single_choice::SingleChoice},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    Human(Color),
    // AI(Color, Difficulty)
    AI(Color, u8),
}

impl Player {
    const POSSIBLE_COLORS: [Color; 4] = [
        colors::rgb(255, 0, 0),
        colors::rgb(0, 255, 0),
        colors::rgb(0, 0, 255),
        colors::rgb(255, 255, 0),
    ];
}

pub struct PlayerCard {
    pub player: Player,
    difficulty_selector: SingleChoice,
}

impl PlayerCard {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            difficulty_selector: SingleChoice::new(
                vec!["easy".to_string(), "medium".to_string(), "hard".to_string()],
                0,
                |l| l,
            ),
        }
    }
}

impl Layout for PlayerCard {
    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        vec![]
    }
}

pub struct PreflightScene {
    back_btn: LabeledButton,
    pub players: Vec<Player>,
    ready_btn: LabeledButton,
}

impl PreflightScene {
    const READY_FONT_SIZE: f32 = 64.0;

    pub fn new_player_color(&self) -> Color {
        *Player::POSSIBLE_COLORS
            .iter()
            .find(|&&color| {
                !self.players.iter().any(|p| match p {
                    Player::Human(c) | Player::AI(c, _) => *c == color,
                })
            })
            .unwrap()
    }
}

impl Default for PreflightScene {
    fn default() -> Self {
        let mut res = Self {
            back_btn: LabeledButton::back_btn(),
            ready_btn: LabeledButton::new_with_id(
                "ready",
                |l| {
                    l.with_align(Align::Center)
                        .with_color(colors::WHITE)
                        .with_font_size(Self::READY_FONT_SIZE)
                        .with_line_height(Self::READY_FONT_SIZE)
                        .with_texture_align((0.5, 0.6))
                },
                |b| b.with_color(colors::color_primary()).with_radius(0.5),
            ),

            players: vec![],
        };
        res.players.push(Player::Human(res.new_player_color()));
        res.players.push(Player::Human(res.new_player_color()));
        res
    }
}

impl Layout for PreflightScene {
    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        LayoutBuilder::new()
            .at_rect(
                super::BACK_BTN_RECT,
                &mut self.back_btn as &mut dyn Component,
            )
            .at_rect(
                (0.2, -0.35, 0.5, 0.15),
                &mut self.ready_btn as &mut dyn Component,
            )
            .build()
    }
}

impl Scene for PreflightScene {
    fn next_scene(&mut self) -> Option<NextScene> {
        if self.back_btn.triggered() {
            return Some(NextScene::Pop);
        }
        if self.ready_btn.triggered() {
            // return Some(NextScene::Push(...));
            todo!()
        }
        None
    }
}
