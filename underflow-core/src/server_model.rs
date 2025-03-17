use crate::board::*;
use crate::board::BoardStat::Ready;
use crate::protocol::CommandContent::{self, *};
use crate::protocol::ResponseContent::{GameOver, Valid, PhaseChange};
use crate::protocol::{ResponseContent, UnderflowError};
use std::collections::VecDeque;

/// An abstraction of server that only runs game process.
pub struct ServerModel {
    player_count: u8,
    board: Board,
    is_flowing_phase: bool,
    player_queue: VecDeque<u8>,
}

/// An ideal server, processing commands and producing responses.
impl ServerModel {
    pub fn new(player_count: u8, size: u8) -> ServerModel {
        let mut player_queue = VecDeque::new();
        for i in 0..player_count {
            player_queue.push_back(i);
        }
        ServerModel {
            player_count,
            board: Board::init(player_count, size),
            is_flowing_phase: false,
            player_queue
        }
    }

    fn get_stat(&self) -> BoardStat {
        self.board.stat()
    }

    /// Judges which response server should return after a valid command arrives.
    /// - `GameOver`: Only one of the players have blocks left.
    /// - `PhaseChange`: Notifies that the stage of putting blocks is end.
    /// - `Valid`: Shows that the command is valid, and does not cause critical effects.
    fn judge(&mut self) -> ResponseContent {
        if let Ready {player_stat, total_occupied} = self.get_stat() {
            if player_stat.len() == 1 {
                GameOver(player_stat.into_iter().next().unwrap().0)
            } else if !self.is_flowing_phase {
                self.is_flowing_phase = true;
                PhaseChange
            } else {
                Valid
            }
        } else {
            Valid
        }
    }

    pub fn process_command(&mut self, command: CommandContent) -> Result<ResponseContent, UnderflowError> {
        match command {
            CommandContent::FlowX(x, pos) => {
                self.board.flow_x(x, pos)?;
                Ok(self.judge())
            }
            CommandContent::FlowY(y, pos) => {
                self.board.flow_x(y, pos)?;
                Ok(self.judge())
            }
            CommandContent::Put(cell, x, y) => {
                self.board.put(cell, x, y)?;
                Ok(self.judge())
            }
            CommandContent::Exchange(id, src_x, src_y, dst_x, dst_y) => {
                
                Ok(Valid)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_easy_game() {
        let mut server = ServerModel::new(2, 2);
        let stat = server.get_stat();
        assert_eq!(stat, BoardStat::NotReady);
        server.process_command(Put(CellState::Occupied(0), 0, 0)).unwrap();
        
        // shall fail
        server.process_command(Put(CellState::Occupied(1), 0, 0)).expect_err("");
        
        server.process_command(Put(CellState::Occupied(1), 0, 1)).unwrap();
        server.process_command(Put(CellState::Occupied(1), 1, 1)).unwrap();
        
        //shall end the phase of placing blocks.
        let res = server.process_command(Put(CellState::Occupied(0), 1, 0)).unwrap();
        
        assert_eq!(res, ResponseContent::PhaseChange);
        let stat = server.get_stat();
        assert_ne!(stat, BoardStat::NotReady);
        assert!(server.is_flowing_phase);
        
        let (dict, _) = stat.unwrap();
        assert_eq!(dict.get(&0).unwrap(), &2);
        
        server.process_command(FlowX(1, true)).unwrap();
        let res = server.process_command(FlowX(1, true)).unwrap();
        assert_eq!(res, ResponseContent::GameOver(0));
    }
}

