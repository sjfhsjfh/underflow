use crate::board::*;
use crate::board::BoardStat::Ready;
#[allow(unused_imports)]
use crate::protocol::CommandContent::{self, *};
use crate::protocol::ResponseContent::{GameOver, Valid, PhaseChange, Elimination};
use crate::protocol::{ResponseContent, UnderflowError};
use std::collections::VecDeque;

/// An abstraction of server that only runs game process.
#[allow(dead_code)]
pub struct ServerModel {
    board: Board,
    is_flowing_phase: bool,
    player_queue: VecDeque<u8>,
}

/// An ideal server, processing commands and producing responses.
impl ServerModel {
    #[allow(dead_code)]
    pub fn new(player_count: u8, size: u8) -> ServerModel {
        let mut player_queue = VecDeque::new();
        for i in 0..player_count {
            player_queue.push_back(i);
        }
        ServerModel {
            board: Board::init(player_count, size),
            is_flowing_phase: false,
            player_queue
        }
    }

    #[allow(dead_code)]
    fn get_stat(&self) -> BoardStat {
        self.board.stat()
    }

    /// Judges which response server should return after a valid command arrives.
    /// - `GameOver`: Only one of the players have blocks left.
    /// - `PhaseChange`: Notifies that the stage of putting blocks is end.
    /// - `Valid`: Shows that the command is valid, and does not cause critical effects.
    #[allow(dead_code)]
    fn judge(&mut self) -> ResponseContent {
        if let Ready {player_stat, .. } = self.get_stat() {
            if player_stat.len() == 1 {
                GameOver(player_stat.into_iter().next().unwrap().0)
            } else if !self.is_flowing_phase {
                self.is_flowing_phase = true;
                
                let done = self.player_queue.pop_front().unwrap();
                self.player_queue.push_back(done);
                PhaseChange(*self.player_queue.front().unwrap())
            } else if player_stat.len() != self.player_queue.len() {
                let mut delete_idx = usize::MAX;
                let mut delete_id = u8::MAX;
                for (i, x) in self.player_queue.iter().enumerate() {
                    if !player_stat.contains_key(&x) {
                        delete_idx = i;
                        delete_id = *x;
                        break;
                    }
                }
                self.player_queue.remove(delete_idx);

                let done = self.player_queue.pop_front().unwrap();
                self.player_queue.push_back(done);
                Elimination(delete_id, *self.player_queue.front().unwrap())
            } else {
                let done = self.player_queue.pop_front().unwrap();
                self.player_queue.push_back(done);
                Valid(*self.player_queue.front().unwrap())
            }
        } else {
            let done = self.player_queue.pop_front().unwrap();
            self.player_queue.push_back(done);
            Valid(*self.player_queue.front().unwrap())
        }
    }

    #[allow(dead_code)]
    pub fn process_command(&mut self, command: CommandContent) -> Result<ResponseContent, UnderflowError> {
        match command {
            CommandContent::FlowX(x, pos) => {
                self.board.flow_x(x, pos)?;
                Ok(self.judge())
            }
            CommandContent::FlowY(y, pos) => {
                self.board.flow_y(y, pos)?;
                Ok(self.judge())
            }
            CommandContent::Put(cell, x, y) => {
                self.board.put(cell, x, y)?;
                Ok(self.judge())
            }
            #[allow(unused_variables)]
            CommandContent::Exchange(id, src_x, src_y, dst_x, dst_y) => {
                
                Ok(self.judge())
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
        server.process_command(Put(CellState::Occupied(0), 1, 0)).unwrap();
        
        //shall end the phase of placing blocks.
        let res = server.process_command(Put(CellState::Occupied(1), 1, 1)).unwrap();
        
        assert_eq!(res, ResponseContent::PhaseChange(0));
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

