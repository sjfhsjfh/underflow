use rand::prelude::*;
use underflow_core::protocol::FlowCommand;
use underflow_core::protocol::GamePhase;
use underflow_core::server::*;

mod heuristic;
mod util;
pub use heuristic::*;
pub use util::*;

/// AI difficulties

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            return MediumStrategy::filling_move(server, commands);
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
        server: &mut FlowServer,
        commands: Vec<FlowCommand>,
    ) -> Result<FlowCommand, OperationError> {
        let mut best_score = i32::MIN;
        let mut best_commands = Vec::new();
        let borad = &server.board;
        let size = borad.size();

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
        if server.phase == GamePhase::Filling {
            return MediumStrategy::filling_move(server, get_valid_commands(server, player_id));
        }

        let depth = match server.player_count() {
            2 => 7,
            3 => 5,
            4 => 4,
            _ => 4, // For more players, reduce depth to avoid long computation
        };

        let (_, command) = HardStrategy::maxn_search(server, player_id, depth);

        command.ok_or(OperationError::NoValidMove)
    }

    fn maxn_search(server: &FlowServer, player_id: u8, depth: u8) -> (f64, Option<FlowCommand>) {
        if depth == 0 || server.game_over() {
            return (heuristic(server, player_id), None);
        }

        let mut best_score = f64::NEG_INFINITY;
        let mut best_command = None;

        // Get all valid commands for the player
        let commands = get_valid_commands(server, player_id);

        // simulate each command
        for cmd in commands {
            let mut new_server = server.clone();
            if new_server.handle(cmd.clone()).is_err() {
                continue; // Skip invalid commands
            }

            // dfs
            let (score, _) = HardStrategy::maxn_search(&new_server, player_id, depth - 1);

            if score > best_score {
                best_score = score;
                best_command = Some(cmd);
            }
        }

        if best_score == f64::NEG_INFINITY {
            return (f64::NEG_INFINITY, None); // No valid moves
        }
        (best_score, best_command)
    }
}
