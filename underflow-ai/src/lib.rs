use rand::prelude::*;
use underflow_core::protocol::FlowCommand;
use underflow_core::protocol::GamePhase;
use underflow_core::server::*;

mod heuristic;
mod util;
pub use heuristic::*;
pub use util::*;

/// AI difficulties

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub struct AI {
    player_id: u8,
    difficulty: Difficulty,
}

impl AI {
    pub fn new(player_id: u8, difficulty: Difficulty) -> Self {
        AI {
            player_id,
            difficulty,
        }
    }

    /// Make a move based on the AI's difficulty level
    pub fn make_move(&self, server: &mut FlowServer) -> Result<FlowCommand, OperationError> {
        match self.difficulty {
            Difficulty::Easy => SimpleStrategy::make_move(self.player_id, server),
            Difficulty::Medium => MediumStrategy::make_move(self.player_id, server),
            Difficulty::Hard => HardStrategy::make_move(self.player_id, server),
        }
    }
}

// ========================
// DIFFICULTY STRATEGY
// ========================

struct SimpleStrategy;

struct MediumStrategy;

struct HardStrategy;

impl SimpleStrategy {
    /// Randomly select and execute a valid command
    pub fn make_move(
        player_id: u8,
        server: &mut FlowServer,
    ) -> Result<FlowCommand, OperationError> {
        // Get all valid commands for the player
        let commands = get_valid_commands(server, player_id);

        // Randomly select one command
        if let Some(cmd) = commands.into_iter().choose(&mut rand::rng()) {
            return Ok(cmd);
        }

        Err(OperationError::NoValidMove)
    }
}

impl MediumStrategy {
    /// Search for the best move using heuristic evaluation
    pub fn make_move(
        player_id: u8,
        server: &mut FlowServer,
    ) -> Result<FlowCommand, OperationError> {
        let mut best_score = f64::NEG_INFINITY;
        let mut best_commands = Vec::new();

        // Get all valid commands for the player
        let commands = get_valid_commands(server, player_id);

        if server.phase == GamePhase::Filling {
            // If in filling phase, use simple strategy
            return MediumStrategy::filling_move(player_id, server);
        }

        // evaluate each command using the heuristic function
        for cmd in commands {
            match try_handle_command(server, player_id, cmd.clone()) {
                Ok(new_server) => {
                    let score = heuristic(&new_server, player_id);
                    if score > best_score {
                        best_score = score;
                        best_commands.clear();
                        best_commands.push(cmd);
                    } else if score == best_score {
                        best_commands.push(cmd);
                    }
                }
                Err(_) => continue,
            }
        }

        // choose a command from the best commands found
        if let Some(cmd) = best_commands.into_iter().choose(&mut rand::rng()) {
            return Ok(cmd);
        }

        // if no valid commands found, fallback to simple strategy
        SimpleStrategy::make_move(player_id, server)
    }

    fn filling_move(
        player_id: u8,
        server: &mut FlowServer,
    ) -> Result<FlowCommand, OperationError> {
        let mut best_score = i32::MIN;
        let mut best_commands = Vec::new();
        let borad = &server.board;
        let size = borad.size();

        // Get all valid commands for the player
        let commands = get_valid_commands(server, player_id);

        // Choose the best command
        for cmd in commands {
            if let FlowCommand::SetOccupied { x, y, .. } = cmd {
                // Check if the cell is empty
                let score = evaluate_filling_position(size as i32, x as i32, y as i32);
                if score > best_score {
                    best_score = score;
                    best_commands.clear();
                    best_commands.push(cmd);
                } else if score == best_score {
                    best_commands.push(cmd);
                }
            }
        }
        // choose a command from the best commands found
        if let Some(cmd) = best_commands.into_iter().choose(&mut rand::rng()) {
            return Ok(cmd);
        }

        return Err(OperationError::NoValidMove);
    }
    
}

impl HardStrategy {
    pub fn make_move(
        player_id: u8,
        server: &mut FlowServer,
    ) -> Result<FlowCommand, OperationError> {
        MediumStrategy::make_move(player_id, server)
    }
}
