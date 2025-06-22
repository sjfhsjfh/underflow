use crate::{
    Board, CellState,
    history::BoardHistory,
    protocol::{FlowCommand, FlowError, FlowResponse, GamePhase},
};

#[derive(Debug, Clone, Copy)]
pub struct FlowServerConfig {
    pub player_count: u8,
    pub size: u8,
}

#[derive(Clone)]
pub struct FlowServer {
    pub board: Board,
    pub current_player: u8,
    history: BoardHistory,
    pub phase: GamePhase,
    player_count: u8,
}

impl FlowServer {
    pub fn new(config: FlowServerConfig) -> Self {
        Self {
            board: Board::init(config.player_count, config.size),
            current_player: 0,
            history: BoardHistory::new(),
            phase: GamePhase::Filling,
            player_count: config.player_count,
        }
    }

    pub fn winning(&self) -> Option<u8> {
        if self.phase != GamePhase::Flowing {
            return None;
        }
        let mut winner = None;
        for state in self.board.get_cells().iter().flat_map(|c| c.iter()) {
            if let CellState::Occupied(player) = state {
                if winner.is_none() {
                    winner = Some(*player);
                } else if winner != Some(*player) {
                    return None; // More than one player has occupied cells
                }
            }
        }
        winner
    }

    pub fn optimal_size(player_count: u8) -> u8 {
        match player_count {
            2 => 6,
            3 => 7,
            4 => 7,
            _ => 8,
        }
    }

    pub fn can_flow_x(&self, y: u8) -> bool {
        y < self.board.size && self.phase.is_flowing() && self.board.can_flow_x(y)
    }

    pub fn can_flow_y(&self, x: u8) -> bool {
        x < self.board.size && self.phase.is_flowing() && self.board.can_flow_y(x)
    }

    pub fn player_count(&self) -> u8 {
        self.player_count
    }

    fn player_alive(&self, player: u8) -> bool {
        self.phase.is_filling()
            || self
                .board
                .cells
                .iter()
                .flat_map(|c| c.iter())
                .any(|&cell| cell == CellState::Occupied(player))
    }

    fn current_player_alive(&self) -> bool {
        self.player_alive(self.current_player)
    }

    fn check_player(&mut self, player: u8) -> FlowResponse {
        if !self.player_alive(player) {
            return Err(FlowError::YouAreDead);
        }

        if self.current_player != player {
            return Err(FlowError::NotYourTurn);
        }

        Ok(())
    }

    fn expect_phase(&self, phase: GamePhase) -> FlowResponse {
        if self.phase != phase {
            return Err(FlowError::InvalidPhase);
        }
        Ok(())
    }

    fn checked_set(&mut self, x: u8, y: u8, state: CellState) -> FlowResponse {
        let mut dry_run = self.board.clone();
        dry_run.set(x, y, state);
        if self.history.is_recurrence(&dry_run) {
            return Err(FlowError::Recurrence);
        }
        self.board.set(x, y, state);
        Ok(())
    }

    fn checked_double_set(
        &mut self,
        pos1: (u8, u8),
        state1: CellState,
        pos2: (u8, u8),
        state2: CellState,
    ) -> FlowResponse {
        let mut dry_run = self.board.clone();
        dry_run.set(pos1.0, pos1.1, state1);
        dry_run.set(pos2.0, pos2.1, state2);
        if self.history.is_recurrence(&dry_run) {
            return Err(FlowError::Recurrence);
        }
        self.board.set(pos1.0, pos1.1, state1);
        self.board.set(pos2.0, pos2.1, state2);
        Ok(())
    }

    pub fn will_be_recurrence(&self, idx: u8, is_x: bool, positive: bool) -> bool {
        let mut dry_run = self.board.clone();
        if is_x {
            if !dry_run.flow_x(idx, positive) {
                return false;
            }
        } else if !dry_run.flow_y(idx, positive) {
            return false;
        }
        self.history.is_recurrence(&dry_run)
    }

    fn checked_flow(&mut self, idx: u8, is_x: bool, positive: bool) -> FlowResponse {
        if idx >= self.board.size {
            return Err(FlowError::IndexOutOfRange);
        }
        let mut dry_run = self.board.clone();
        if is_x {
            if !dry_run.flow_x(idx, positive) {
                return Err(FlowError::BlockedByAnchor);
            }
        } else if !dry_run.flow_y(idx, positive) {
            return Err(FlowError::BlockedByAnchor);
        }
        if self.history.is_recurrence(&dry_run) {
            return Err(FlowError::Recurrence);
        }
        if is_x {
            self.board.flow_x(idx, positive);
        } else {
            self.board.flow_y(idx, positive);
        }
        self.history.push(&self.board);
        Ok(())
    }

    fn next_player(&mut self) {
        self.current_player += 1;
        self.current_player %= self.player_count;
    }

    fn next_player_alive(&mut self) {
        self.next_player();
        while !self.current_player_alive() {
            self.next_player();
        }
    }

    fn last_player(&mut self) {
        self.current_player += self.player_count - 1;
        self.current_player %= self.player_count;
    }

    pub fn handle(&mut self, cmd: FlowCommand) -> FlowResponse {
        match cmd {
            FlowCommand::FlowX {
                player,
                y,
                positive,
            } => {
                self.check_player(player)?;
                self.expect_phase(GamePhase::Flowing)?;
                self.checked_flow(y, true, positive)?;
                self.next_player_alive();
            }
            FlowCommand::FlowY {
                player,
                x,
                positive,
            } => {
                self.check_player(player)?;
                self.expect_phase(GamePhase::Flowing)?;
                self.checked_flow(x, false, positive)?;
                self.next_player_alive();
            }
            FlowCommand::SetAnchor { player, x, y } => {
                self.check_player(player)?;
                self.expect_phase(GamePhase::Flowing)?;
                if self.board.is_occupied(x, y) {
                    return Err(FlowError::AlreadyOccupied);
                }
                let old = self
                    .board
                    .get_cells()
                    .iter()
                    .enumerate()
                    .flat_map(|(x, col)| col.iter().enumerate().map(move |(y, cell)| (x, y, *cell)))
                    .find(|(_, _, cell)| *cell == CellState::Anchored(player))
                    .map(|(x, y, _)| (x as u8, y as u8));
                if let Some(old) = old {
                    self.checked_double_set(
                        old,
                        CellState::Neutral,
                        (x, y),
                        CellState::Anchored(player),
                    )?;
                } else {
                    self.checked_set(x, y, CellState::Anchored(player))?;
                }
                self.next_player_alive();
            }
            FlowCommand::SetOccupied { player, x, y } => {
                self.check_player(player)?;
                self.expect_phase(GamePhase::Filling)?;
                if self.board.is_occupied(x, y) || self.board.is_neutral(x, y) {
                    return Err(FlowError::AlreadyOccupied);
                }
                self.checked_set(x, y, CellState::Occupied(player))?;
                self.last_player();
                if self.board.is_ready() {
                    self.phase = GamePhase::Flowing;
                    self.history.push(&self.board);
                }
            }
        }

        let dead_anchors: Vec<_> = self
            .board
            .get_cells()
            .iter()
            .enumerate()
            .flat_map(|(x, col)| col.iter().enumerate().map(move |(y, cell)| (x, y, *cell)))
            .filter_map(|(x, y, cell)| {
                if let CellState::Anchored(player) = cell {
                    if !self.player_alive(player) {
                        Some((x as u8, y as u8))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        for (x, y) in dead_anchors {
            self.board.set(x, y, CellState::Neutral);
        }
        Ok(())
    }
}
