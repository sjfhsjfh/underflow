use crate::{
    Board, CellState,
    history::BoardHistory,
    protocol::{FlowCommand, FlowError, FlowResponse, GamePhase},
};

pub struct FlowServer {
    board: Board,
    current_player: u8,
    history: BoardHistory,
    phase: GamePhase,
    player_count: u8,
}

impl FlowServer {
    fn check_player(&self, player: u8) -> FlowResponse {
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
        if self.phase.is_flowing() {
            self.history.push(&self.board);
        }
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
                if y >= self.board.size {
                    return Err(FlowError::IndexOutOfRange);
                }
                if !self.board.flow_x(y, positive) {
                    return Err(FlowError::BlockedByAnchor);
                }
                self.next_player();
            }
            FlowCommand::FlowY {
                player,
                x,
                positive,
            } => {
                self.check_player(player)?;
                if x >= self.board.size {
                    return Err(FlowError::IndexOutOfRange);
                }
                if !self.board.flow_y(x, positive) {
                    return Err(FlowError::BlockedByAnchor);
                }
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
            }
        }
        Ok(())
    }
}
