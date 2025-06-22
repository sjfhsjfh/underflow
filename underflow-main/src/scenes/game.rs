use std::iter::once;

use comui::{
    component::Component,
    components::label::{Align, Label},
    layout::{Layout, LayoutBuilder},
    scene::{NextScene, Scene},
    shading::IntoShading,
    utils::Transform,
    window::Window,
};
use lyon::{
    geom::euclid::Point2D,
    math::{Box2D, point},
    path::{Path, Polygon, Winding},
};
use macroquad::{input::TouchPhase, prelude::Touch};
use nalgebra::Vector2;
use rand::seq::SliceRandom;
use underflow_ai::AI;
use underflow_core::{
    CellState,
    protocol::{FlowCommand, GamePhase},
    server::{FlowServer, FlowServerConfig},
};

use crate::{
    colors, components::button::LabeledButton, scenes::preflight::Player, tl, utils::UTransform,
};

pub struct BoardComponent {
    /// data, availability
    pub cells: Vec<Vec<CellState>>,
    /// availability of the flow buttons, order: top, left, right, bottom
    pub flow_btns: [Vec<bool>; 4],

    pub color_map: Vec<Player>,

    touch_scaling: (f32, f32),
    /// Tracking touch id and grid coord
    touch_state: Option<(u64, (usize, usize))>,
    triggered_grid: Option<GridElem>,
}

impl BoardComponent {
    const CELL_GAP: f32 = 0.1;
    const BTN_GAP_RATIO: f32 = 5.5;

    fn board_length(&self) -> usize {
        self.cells.len()
    }

    fn grid_coord_to_elem(&self, x: usize, y: usize) -> GridElem {
        let l = self.board_length();
        if x == 0 {
            return GridElem::LeftBtn { y: y as u8 - 1 };
        }
        if x == l + 1 {
            return GridElem::RightBtn { y: y as u8 - 1 };
        }
        if y == 0 {
            return GridElem::BottomBtn { x: x as u8 - 1 };
        }
        if y == l + 1 {
            return GridElem::TopBtn { x: x as u8 - 1 };
        }
        GridElem::Cell {
            x: x as u8 - 1,
            y: y as u8 - 1,
        }
    }

    fn empty_color() -> colors::Color {
        colors::rgb(162, 162, 162)
    }

    fn neutral_color() -> colors::Color {
        colors::rgb(103, 103, 103)
    }

    fn btn_color() -> colors::Color {
        colors::rgb(65, 65, 65)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GridElem {
    Cell { x: u8, y: u8 },
    TopBtn { x: u8 },
    LeftBtn { y: u8 },
    RightBtn { y: u8 },
    BottomBtn { x: u8 },
}

impl GridElem {
    pub fn to_cmd(self, phase: GamePhase, player: u8) -> FlowCommand {
        match self {
            Self::Cell { x, y } => {
                if phase.is_filling() {
                    FlowCommand::SetOccupied { player, x, y }
                } else {
                    FlowCommand::SetAnchor { player, x, y }
                }
            }
            Self::TopBtn { x } => FlowCommand::FlowY {
                player,
                x,
                positive: false,
            },
            Self::LeftBtn { y } => FlowCommand::FlowX {
                player,
                y,
                positive: true,
            },
            Self::RightBtn { y } => FlowCommand::FlowX {
                player,
                y,
                positive: false,
            },
            Self::BottomBtn { x } => FlowCommand::FlowY {
                player,
                x,
                positive: true,
            },
        }
    }
}

impl Component for BoardComponent {
    fn render(&mut self, tr: &Transform, target: &mut Window) {
        let abs_w = tr.transform_vector(&Vector2::new(1.0, 0.0)).norm();
        let abs_h = tr.transform_vector(&Vector2::new(0.0, 1.0)).norm();
        // tr * sub_tr maps (-0.5, -0.5) - (0.5, 0.5) to the square on screen
        let sub_tr = if abs_w > abs_h {
            self.touch_scaling = (abs_w / abs_h, 1.0);
            Transform::new_nonuniform_scaling(&Vector2::new(abs_h / abs_w, 1.0))
        } else {
            self.touch_scaling = (1.0, abs_h / abs_w);
            Transform::new_nonuniform_scaling(&Vector2::new(1.0, abs_w / abs_h))
        };
        let new_tr = UTransform::new(tr * sub_tr);

        // Draw flow btns
        let mut btn_builder = Path::builder();
        let btn_size =
            1.0 / (1.0 + Self::CELL_GAP * Self::BTN_GAP_RATIO) / (self.board_length() + 2) as f32;
        for (x, _) in self.flow_btns[0].iter().enumerate().filter(|(_, d)| **d) {
            let center_x = (x as f32 + 1.5) / (self.board_length() + 2) as f32 - 0.5;
            let base_y = 0.5 - btn_size / 2.0 * Self::CELL_GAP * Self::BTN_GAP_RATIO;
            btn_builder.add_polygon(Polygon {
                closed: true,
                points: &[
                    point(center_x - btn_size / 2.0, base_y),
                    point(center_x, base_y - f32::sqrt(3.0) / 2.0 * btn_size),
                    point(center_x + btn_size / 2.0, base_y),
                ],
            });
        }
        for (x, _) in self.flow_btns[3].iter().enumerate().filter(|(_, d)| **d) {
            let center_x = (x as f32 + 1.5) / (self.board_length() + 2) as f32 - 0.5;
            let base_y = -0.5 + btn_size / 2.0 * Self::CELL_GAP * Self::BTN_GAP_RATIO;
            btn_builder.add_polygon(Polygon {
                closed: true,
                points: &[
                    point(center_x - btn_size / 2.0, base_y),
                    point(center_x, base_y + f32::sqrt(3.0) / 2.0 * btn_size),
                    point(center_x + btn_size / 2.0, base_y),
                ],
            });
        }
        for (y, _) in self.flow_btns[1].iter().enumerate().filter(|(_, d)| **d) {
            let center_y = (y as f32 + 1.5) / (self.board_length() + 2) as f32 - 0.5;
            let base_x = -0.5 + btn_size / 2.0 * Self::CELL_GAP * Self::BTN_GAP_RATIO;
            btn_builder.add_polygon(Polygon {
                closed: true,
                points: &[
                    point(base_x, center_y - btn_size / 2.0),
                    point(base_x + f32::sqrt(3.0) / 2.0 * btn_size, center_y),
                    point(base_x, center_y + btn_size / 2.0),
                ],
            });
        }
        for (y, _) in self.flow_btns[2].iter().enumerate().filter(|(_, d)| **d) {
            let center_y = (y as f32 + 1.5) / (self.board_length() + 2) as f32 - 0.5;
            let base_x = 0.5 - btn_size / 2.0 * Self::CELL_GAP * Self::BTN_GAP_RATIO;
            btn_builder.add_polygon(Polygon {
                closed: true,
                points: &[
                    point(base_x, center_y - btn_size / 2.0),
                    point(base_x - f32::sqrt(3.0) / 2.0 * btn_size, center_y),
                    point(base_x, center_y + btn_size / 2.0),
                ],
            });
        }

        target.fill_path(
            &btn_builder.build().transformed(&new_tr),
            Self::btn_color().into_shading(),
            1.0,
        );

        // Draw cells
        let mut anchors = vec![];
        let mut builders: Vec<_> = self
            .color_map
            .iter()
            .map(|p| (p.color(), Path::builder()))
            .collect();
        let mut empty_builder = Path::builder();
        let mut neutral_builder = Path::builder();
        let board_size = self.board_length();
        (0..(board_size.pow(2)))
            .map(|i| (i % board_size, i / board_size))
            .for_each(|(x, y)| {
                let builder = match self.cells[x][y] {
                    CellState::Occupied(id) => &mut builders[id as usize].1,
                    CellState::Empty => &mut empty_builder,
                    CellState::Neutral => &mut neutral_builder,
                    CellState::Anchored(id) => {
                        anchors.push((x, y));
                        &mut builders[id as usize].1
                    }
                };
                let cell_size = 1.0 / (1.0 + Self::CELL_GAP) / (board_size + 2) as f32;
                let center_x = (x as f32 + 1.5) / (board_size + 2) as f32 - 0.5;
                let center_y = (y as f32 + 1.5) / (board_size + 2) as f32 - 0.5;
                builder.add_rectangle(
                    &Box2D::new(
                        Point2D::new(center_x - cell_size * 0.5, center_y - cell_size * 0.5),
                        Point2D::new(center_x + cell_size * 0.5, center_y + cell_size * 0.5),
                    ),
                    Winding::Positive,
                );
            });
        for (color, builder) in builders
            .into_iter()
            .chain(once((Self::empty_color(), empty_builder)))
            .chain(once((Self::neutral_color(), neutral_builder)))
        {
            let path = builder.build();
            target.fill_path(
                &path.transformed(&new_tr),
                color.with_alpha(0.8).into_shading(),
                1.0,
            );
        }

        // Draw anchors
        let mut builder = Path::builder();
        for (x, y) in anchors {
            let cell_size = 1.0 / (1.0 + Self::CELL_GAP) / (board_size + 2) as f32;
            let center_x = (x as f32 + 1.5) / (board_size + 2) as f32 - 0.5;
            let center_y = (y as f32 + 1.5) / (board_size + 2) as f32 - 0.5;
            builder.add_circle(
                point(center_x, center_y),
                cell_size * 0.4,
                Winding::Positive,
            );
        }
        target.stroke_path(
            &builder.build().transformed(&new_tr),
            colors::BLACK.into_shading(),
            1.0,
            3.0,
        );
    }

    fn touch(&mut self, touch: &Touch) -> anyhow::Result<bool> {
        if self.triggered_grid.is_some() {
            return Ok(false);
        }
        let (x, y) = (
            (touch.position.x * self.touch_scaling.0),
            (touch.position.y * self.touch_scaling.1),
        );
        if !((-0.5..=0.5).contains(&x) && (-0.5..=0.5).contains(&y)) {
            return Ok(false);
        }
        let l = self.board_length();
        let (grid_x, grid_y) = (
            ((x + 0.5) * (l + 2) as f32).floor() as usize,
            ((y + 0.5) * (l + 2) as f32).floor() as usize,
        );
        if (grid_x, grid_y) == (0, 0)
            || (grid_x, grid_y) == (0, l + 1)
            || (grid_x, grid_y) == (l + 1, 0)
            || (grid_x, grid_y) == (l + 1, l + 1)
        {
            return Ok(false);
        }
        let should_consume = match touch.phase {
            TouchPhase::Started => {
                self.touch_state = Some((touch.id, (grid_x, grid_y)));
                false
            }
            TouchPhase::Moved | TouchPhase::Stationary => {
                if self
                    .touch_state
                    .is_some_and(|(tid, (x, y))| tid == touch.id && (x, y) != (grid_x, grid_y))
                {
                    self.touch_state = None;
                }
                false
            }
            TouchPhase::Cancelled => {
                self.touch_state = None;
                false
            }
            TouchPhase::Ended => self.touch_state == Some((touch.id, (grid_x, grid_y))),
        };
        if should_consume {
            self.triggered_grid = Some(self.grid_coord_to_elem(grid_x, grid_y));
        }
        Ok(should_consume)
    }
}

pub struct GameScene {
    pub players: Vec<Player>,
    pub game_server: FlowServer,
    pub board: BoardComponent,
    hint: Label,
    pub pause_btn: LabeledButton,
    next_scene: Option<NextScene>,
}

impl GameScene {
    const HINT_SIZE: f32 = 96.;

    pub fn current_player(&self) -> &Player {
        &self.players[self.game_server.current_player as usize]
    }

    pub fn current_player_color(&self) -> colors::Color {
        self.current_player().color()
    }

    pub fn new(mut players: Vec<Player>) -> Self {
        players.shuffle(&mut rand::rng());
        let player_count = players.len() as u8;
        let size = FlowServer::optimal_size(player_count);
        let game_server = FlowServer::new(FlowServerConfig { player_count, size });
        let board = BoardComponent {
            cells: game_server.board.get_cells().clone(),
            flow_btns: [
                vec![false; size as usize],
                vec![false; size as usize],
                vec![false; size as usize],
                vec![false; size as usize],
            ],
            color_map: players.clone(),

            touch_scaling: (1.0, 1.0),
            touch_state: None,
            triggered_grid: None,
        };
        Self {
            players,
            game_server,
            board,
            hint: Label::new(tl!("your-turn"))
                .with_align(Align::Right)
                .with_font_size(Self::HINT_SIZE)
                .with_line_height(Self::HINT_SIZE)
                .with_texture_align((1.0, 0.0)),
            pause_btn: LabeledButton::pause_btn(),
            next_scene: None,
        }
    }
}

impl Layout for GameScene {
    fn before_render(&mut self, _: &Transform, _: &mut Window) {
        self.hint.color = self.current_player_color();
        if self.hint.text != tl!("your-turn") {
            self.hint.text = tl!("your-turn").into_owned();
        }
        self.board.cells = self.game_server.board.get_cells().clone();
        let len = self.board.board_length() as u8;
        self.board.flow_btns = [
            (0..len)
                .map(|x| {
                    self.game_server.can_flow_y(x)
                        && !self.game_server.will_be_recurrence(x, false, false)
                })
                .collect(),
            (0..len)
                .map(|y| {
                    self.game_server.can_flow_x(y)
                        && !self.game_server.will_be_recurrence(y, true, true)
                })
                .collect(),
            (0..len)
                .map(|y| {
                    self.game_server.can_flow_x(y)
                        && !self.game_server.will_be_recurrence(y, true, false)
                })
                .collect(),
            (0..len)
                .map(|x| {
                    self.game_server.can_flow_y(x)
                        && !self.game_server.will_be_recurrence(x, false, true)
                })
                .collect(),
        ];
        if let Player::AI(_, diff) = self.current_player() {
            // TODO: make this async...
            let cmd = AI::new(self.game_server.current_player, *diff)
                .make_move(&mut self.game_server)
                .unwrap();
            self.game_server.handle(cmd).unwrap();
        }
    }

    fn components(&mut self) -> Vec<(Transform, &mut dyn Component)> {
        LayoutBuilder::new()
            .at_rect(super::BACK_BTN_RECT, &mut self.pause_btn)
            .at_rect((0.0, 0.0, 0.9, 0.7), &mut self.board)
            .at_rect((0.45, 0.45, 0.5, 0.5), &mut self.hint)
            .build()
    }

    fn after_render(&mut self, _: &Transform, _: &mut Window) {
        if self.pause_btn.triggered() {
            self.next_scene = None;
            todo!()
        }
        if let Some(g) = self.board.triggered_grid {
            self.board.triggered_grid = None;
            if self.current_player().is_human() {
                let cmd = g.to_cmd(self.game_server.phase, self.game_server.current_player);
                let res = self.game_server.handle(cmd);
                if let Err(e) = res {
                    println!("Error handling command: {:?}", e);
                }
            }
        }
    }
}

impl Scene for GameScene {
    fn next_scene(&mut self) -> Option<comui::scene::NextScene> {
        self.next_scene.take()
    }
}
