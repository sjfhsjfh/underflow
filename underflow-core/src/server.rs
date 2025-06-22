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

    fn current_player_alive(&self) -> bool {
        self.board
            .cells
            .iter()
            .flat_map(|c| c.iter())
            .any(|&cell| cell == CellState::Occupied(self.current_player))
    }

    fn check_player(&mut self, player: u8) -> FlowResponse {
        while !self.current_player_alive() {
            if self.current_player == player {
                return Err(FlowError::YouAreDead);
            }
            self.next_player();
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
                self.next_player();
            }
            FlowCommand::FlowY {
                player,
                x,
                positive,
            } => {
                self.check_player(player)?;
                self.expect_phase(GamePhase::Flowing)?;
                self.checked_flow(x, false, positive)?;
                self.next_player();
            }
            FlowCommand::SetAnchor { player, x, y } => {
                self.check_player(player)?;
                self.expect_phase(GamePhase::Flowing)?;
                if self.board.is_occupied(x, y) {
                    return Err(FlowError::AlreadyOccupied);
                }
                self.checked_set(x, y, CellState::Anchored(player))?;
                self.next_player();
            }
            FlowCommand::SetOccupied { player, x, y } => {
                self.check_player(player)?;
                self.expect_phase(GamePhase::Filling)?;
                if self.board.is_occupied(x, y) {
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
        Ok(())
    }
}
