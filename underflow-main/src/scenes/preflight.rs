use comui::{
    component::Component,
    components::{DataComponent, label::Align},
    layout::{Layout, LayoutBuilder},
    scene::{NextScene, Scene},
    utils::Transform,
};
use macroquad::color::Color;
use underflow_ai::Difficulty;

use crate::{
    colors,
    components::{
        button::{CancelButton, LabeledButton},
        rounded_rect::RoundedRect,
        single_choice::SingleChoice,
    },
    scenes::game::GameScene,
    tl,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    Human(Color),
    AI(Color, Difficulty),
}

impl Player {
    const POSSIBLE_COLORS: [Color; 4] = [
        colors::rgb(157, 0, 0),
        colors::rgb(3, 103, 0),
        colors::rgb(0, 34, 127),
        colors::rgb(135, 81, 0),
    ];

    pub fn color(&self) -> Color {
        match self {
            Player::Human(c) => *c,
            Player::AI(c, _) => *c,
        }
    }

    pub fn match_id(&self, id: &str) -> bool {
        match self {
            Player::Human(_) => id == tl!("player"),
            Player::AI(_, difficulty) => match difficulty {
                Difficulty::Easy => id == tl!("ai-easy"),
                Difficulty::Medium => id == tl!("ai-medium"),
                Difficulty::Hard => id == tl!("ai-hard"),
            },
        }
    }

    pub fn is_human(&self) -> bool {
        matches!(self, Player::Human(_))
    }
}

pub struct PlayerCard {
    pub player: Player,
    cancel_btn: CancelButton,
    difficulty_selector: SingleChoice,
}

impl PlayerCard {
    const CANCEL_BTN_SIZE: f32 = 0.1;
    const CANCEL_BTN_MARGIN: f32 = 0.1;

    const FONT_SIZE: f32 = 36.0;
    const PLAYER_OPTIONS: [&str; 4] = ["player", "ai-easy", "ai-medium", "ai-hard"];

    pub fn new(player: Player) -> Self {
        Self {
            player,
            cancel_btn: CancelButton::builder()
                .with_container(
                    RoundedRect::builder()
                        .with_radius_rel(0.1, 0.1)
                        .with_stroke(player.color().with_alpha(0.8), 2.0),
                )
                .with_stroke((2.0, player.color().with_alpha(0.8)))
                .build(),
            difficulty_selector: SingleChoice::new(
                Self::PLAYER_OPTIONS
                    .iter()
                    .map(|k| tl!(*k).into_owned())
                    .collect(),
                0,
                |l| {
                    l.with_color(player.color().with_alpha(0.8))
                        .with_font_size(Self::FONT_SIZE)
                        .with_line_height(Self::FONT_SIZE)
                        .with_align(Align::Center)
                        .with_texture_align((0.5, 0.6))
                },
            ),
        }
    }

    pub fn canceled(&mut self) -> bool {
        self.cancel_btn.canceled()
    }
}

impl Layout for PlayerCard {
    fn before_render(&mut self, tr: &Transform, target: &mut comui::window::Window) {
        self.difficulty_selector.updated();
        RoundedRect::builder()
            .with_radius_rel(0.03, 0.05)
            .with_fill_color(self.player.color().with_alpha(0.2))
            .with_stroke(self.player.color().with_alpha(0.6), 1.5)
            .build()
            .render(tr, target);

        let id = self.difficulty_selector.get_data().as_str();
        if !self.player.match_id(id) {
            if id == tl!("player") {
                self.player = Player::Human(self.player.color());
            } else if id == tl!("ai-easy") {
                self.player = Player::AI(self.player.color(), Difficulty::Easy);
            } else if id == tl!("ai-medium") {
                self.player = Player::AI(self.player.color(), Difficulty::Medium);
            } else if id == tl!("ai-hard") {
                self.player = Player::AI(self.player.color(), Difficulty::Hard);
            } else {
                unreachable!();
            }
        }
    }

    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        LayoutBuilder::new()
            .at_rect(
                (
                    0.5 - Self::CANCEL_BTN_MARGIN - 0.5 * Self::CANCEL_BTN_SIZE,
                    0.5 - Self::CANCEL_BTN_MARGIN - 0.5 * Self::CANCEL_BTN_SIZE,
                    Self::CANCEL_BTN_SIZE,
                    Self::CANCEL_BTN_SIZE,
                ),
                &mut self.cancel_btn,
            )
            .at_rect((0.0, -0.1, 0.8, 0.7), &mut self.difficulty_selector)
            .build()
    }
}

pub struct PreflightScene {
    back_btn: LabeledButton,
    pub players: Vec<PlayerCard>,
    add_player_btn: LabeledButton,
    ready_btn: LabeledButton,
}

impl PreflightScene {
    const MAX_PLAYERS: usize = 4;
    const MIN_PLAYERS: usize = 2;

    const READY_FONT_SIZE: f32 = 64.0;

    const PLAYER_CARD_WIDTH: f32 = 0.15;
    const PLAYER_CARD_COL_COUNT: u8 = 4;
    const PLAYER_CARD_MARGIN_X: f32 = 0.1;
    const PLAYER_CARD_FIRST_X: f32 =
        -0.5 + Self::PLAYER_CARD_MARGIN_X + 0.5 * Self::PLAYER_CARD_WIDTH;
    const PLAYER_CARD_GRID_WIDTH: f32 =
        (1.0 - 2.0 * Self::PLAYER_CARD_MARGIN_X - Self::PLAYER_CARD_WIDTH)
            / (Self::PLAYER_CARD_COL_COUNT as f32 - 1.0);

    pub fn new_player_color(&self) -> Color {
        *Player::POSSIBLE_COLORS
            .iter()
            .find(|&&color| {
                !self.players.iter().any(|p| match p.player {
                    Player::Human(c) | Player::AI(c, _) => c == color,
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
            add_player_btn: LabeledButton::new_with_id(
                "plus",
                |l| {
                    l.with_align(Align::Center)
                        .with_color(colors::color_secondary())
                        .with_font_size(Self::READY_FONT_SIZE)
                        .with_line_height(Self::READY_FONT_SIZE)
                        .with_texture_align((0.5, 0.6))
                },
                |b| {
                    b.with_color(colors::color_primary().with_alpha(0.5))
                        .with_radius(0.1)
                },
            ),

            players: vec![],
        };
        res.players
            .push(PlayerCard::new(Player::Human(res.new_player_color())));
        res.players
            .push(PlayerCard::new(Player::Human(res.new_player_color())));
        res
    }
}

impl Layout for PreflightScene {
    fn before_render(&mut self, _: &Transform, _: &mut comui::window::Window) {
        if self.add_player_btn.triggered() && self.players.len() < Self::MAX_PLAYERS {
            self.players
                .push(PlayerCard::new(Player::Human(self.new_player_color())));
        }

        let player_count = self.players.len();
        if let Some(idx) = self
            .players
            .iter_mut()
            .enumerate()
            .map(|(idx, p)| (idx, p.canceled()))
            .find(|(_, canceled)| *canceled)
            .map(|(idx, _)| idx)
        {
            if player_count > Self::MIN_PLAYERS {
                self.players.remove(idx);
            }
        }
    }

    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        fn card_rect(idx: usize) -> (f32, f32, f32, f32) {
            let col = idx % PreflightScene::PLAYER_CARD_COL_COUNT as usize;
            let row = idx / PreflightScene::PLAYER_CARD_COL_COUNT as usize;
            (
                PreflightScene::PLAYER_CARD_FIRST_X
                    + col as f32 * PreflightScene::PLAYER_CARD_GRID_WIDTH,
                0.2 - row as f32 * 0.2,
                PreflightScene::PLAYER_CARD_WIDTH,
                0.18,
            )
        }
        let builder = LayoutBuilder::new()
            .at_rect(super::BACK_BTN_RECT, &mut self.back_btn)
            .at_rect((0.2, -0.35, 0.5, 0.15), &mut self.ready_btn);
        let mut last_idx = 0;
        let builder = self
            .players
            .iter_mut()
            .enumerate()
            .fold(builder, |builder, (idx, card)| {
                last_idx = idx;
                builder.at_rect(card_rect(idx), card)
            });
        let builder = if last_idx + 1 < Self::MAX_PLAYERS {
            builder.at_rect(card_rect(last_idx + 1), &mut self.add_player_btn)
        } else {
            builder
        };
        builder.build()
    }
}

impl Scene for PreflightScene {
    fn next_scene(&mut self) -> Option<NextScene> {
        if self.back_btn.triggered() {
            return Some(NextScene::Pop);
        }
        if self.ready_btn.triggered() {
            return Some(NextScene::Replace(Box::new(GameScene::new(
                self.players.iter().map(|p| p.player).collect(),
            )) as Box<dyn Scene>));
        }
        None
    }
}
