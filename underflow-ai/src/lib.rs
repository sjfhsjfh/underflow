use rand::prelude::*;
use std::{
    cell,
    collections::{HashMap, HashSet},
    hash::Hash,
};
use underflow_core::{Board, CellState, FlowError};

/// AI difficulties

pub enum OperationError {
    NoValidMove,
    FlowError(FlowError),
}

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
    pub fn make_move(&self, board: &mut Board) -> Result<(), OperationError> {
        match self.difficulty {
            Difficulty::Easy => SimpleStrategy::make_move(self.player_id, board),
            Difficulty::Medium => MediumStrategy::make_move(self.player_id, board),
            Difficulty::Hard => HardStrategy::make_move(self.player_id, board),
        }
    }
}

// ========================
// ACTION TYPES
// ========================

#[derive(Debug, Clone)]
pub enum FlowOperation {
    /// Horizontal flow (y, direction)
    Horizontal(u8, bool),
    /// Vertical flow (x, direction)
    Vertical(u8, bool),
}

impl FlowOperation {
    /// Executes the flow operation on the game board
    fn excute(&self, board: &mut Board) -> Result<(), OperationError> {
        match self {
            FlowOperation::Horizontal(y, positive) => board
                .flow_x(*y, *positive)
                .map_err(OperationError::FlowError),
            FlowOperation::Vertical(x, positive) => board
                .flow_y(*x, *positive)
                .map_err(OperationError::FlowError),
        }
    }

    /// Return the operation ready to be executed
    fn return_operation(&self) -> Self {
        self.clone()
    }
}

#[derive(Debug, Clone)]

pub enum Action {
    Flow(FlowOperation),
    MoveAnchor(u8, u8, u8, u8), // (from_x, from_y, to_x, to_y)
    PlaceAnchor(u8, u8),
}

impl Action {
    /// Executes the action on the game board
    pub fn execute(&self, board: &mut Board, player_id: u8) -> Result<(), OperationError> {
        match self {
            Action::Flow(flow_op) => flow_op.excute(board),
            Action::MoveAnchor(from_x, from_y, to_x, to_y) => {
                board.set(*from_x, *from_y, CellState::Neutral);
                board.set(*to_x, *to_y, CellState::Anchored(player_id));
                Ok(())
            }
            Action::PlaceAnchor(x, y) => {
                board.set(*x, *y, CellState::Anchored(player_id));
                Ok(())
            }
        }
    }

    /// Return the action ready to be executed
    pub fn return_action(&self) -> Self {
        self.clone()
    }
}

// ========================
// UTILITY FUNCTIONS
// ========================

/// Get all valid flow operations on the current board
fn get_valid_flows(board: &Board) -> Vec<FlowOperation> {
    let size = board.size();
    let mut valid_flows = Vec::new();

    for i in 0..size {
        // Check horizontal flows
        if board.can_flow_x(i) {
            valid_flows.push(FlowOperation::Horizontal(i, true)); // Right flow
            valid_flows.push(FlowOperation::Horizontal(i, false)); // Left flow
        }
        // Check vertical flows
        if board.can_flow_y(i) {
            valid_flows.push(FlowOperation::Vertical(i, true)); // Down flow
            valid_flows.push(FlowOperation::Vertical(i, false)); // Up flow
        }
    }

    valid_flows
}

/// Get all valid anchor positions on the current board
fn get_valid_anchor_positions(board: &Board) -> Vec<(u8, u8)> {
    let size = board.size();
    (0..size)
        .flat_map(|x| (0..size).map(move |y| (x, y)))
        .filter(|&(x, y)| matches!(board.get(x, y), CellState::Neutral))
        .collect()
}

/// Get the position of the anchor for the given player
fn get_anchor_position(board: &Board, player_id: u8) -> Option<(u8, u8)> {
    let size = board.size();
    (0..size)
        .flat_map(|x| (0..size).map(move |y| (x, y)))
        .find(|&(x, y)| {
            matches! (
                board.get(x, y),
                CellState::Anchored(id) if id == player_id
            )
        })
}

/// Get all actions about placing or moving the anchor
fn get_anchor_actions(board: &Board, player_id: u8) -> Vec<Action> {
    let current_anchor = get_anchor_position(board, player_id);
    let valid_positions = get_valid_anchor_positions(board);
    let mut actions = Vec::new();

    if let Some((from_x, from_y)) = current_anchor {
        // Anchor already exists: generate move actions
        for &(to_x, to_y) in &valid_positions {
            actions.push(Action::MoveAnchor(from_x, from_y, to_x, to_y));
        }
    } else {
        // No anchor yet: generate place actions
        for &(x, y) in &valid_positions {
            actions.push(Action::PlaceAnchor(x, y));
        }
    }

    actions
}

// Get anchored cells
fn get_anchor_lock_state(board: &Board) -> (HashSet<u8>, HashSet<u8>, HashSet<(u8, u8)>) {
    let mut locked_rows = HashSet::new();
    let mut locked_cols = HashSet::new();
    let mut locked_cells = HashSet::new();
    let mut anchor_positions = HashSet::new();
    let size = board.size();

    // Get all existed anchors positions
    for x in 0..size {
        for y in 0..size {
            if let CellState::Anchored(_) = board.get(x, y) {
                anchor_positions.insert((x, y));
            }
        }
    }

    // Calculate lock state
    let anchor_positions = anchor_positions.iter().cloned().collect::<Vec<_>>();
    for &(anchor_x, anchor_y) in &anchor_positions {
        // row/col lock
        locked_rows.insert(anchor_y);
        locked_cols.insert(anchor_x);

        // cell lock
        for other_anchor in &anchor_positions {
            if anchor_x != other_anchor.0 && anchor_y != other_anchor.1 {
                // lock the intersection formed by the two anchors
                locked_cells.insert((anchor_x, anchor_y));
                locked_cells.insert((other_anchor.0, anchor_y));
                locked_cells.insert((anchor_x, other_anchor.1));
                locked_cells.insert((other_anchor.0, other_anchor.1));
            }
        }
    }

    (locked_rows, locked_cols, locked_cells)
}

// ========================
// DIFFICULTY STRATEGY
// ========================

struct SimpleStrategy;

struct MediumStrategy;

struct HardStrategy;

impl SimpleStrategy {
    /// Randomly actions strategy
    pub fn make_move(player_id: u8, board: &mut Board) -> Result<(), OperationError> {
        let flow_operations = get_valid_flows(board);
        if let Some(flow_op) = flow_operations.into_iter().choose(&mut rand::rng()) {
            return flow_op.excute(board);
        }

        let anchor_actions = get_anchor_actions(board, player_id);
        if let Some(action) = anchor_actions.into_iter().choose(&mut rand::rng()) {
            return action.execute(board, player_id);
        }
        Err(OperationError::NoValidMove)
    }
}

impl MediumStrategy {
    /// search for the best flow operation
    pub fn make_move(player_id: u8, board: &mut Board) -> Result<(), OperationError> {
        let mut all_actions = Vec::new();
        let mut best_score = f64::NEG_INFINITY;
        let mut best_action = Vec::new();

        let flow_operations = get_valid_flows(board)
            .into_iter()
            .map(|op| Action::Flow(op.return_operation()));
        all_actions.extend(flow_operations);

        let anchor_actions = get_anchor_actions(board, player_id);
        all_actions.extend(anchor_actions);

        for action in all_actions {
            let mut cloned_board = board.clone();
            if action.execute(&mut cloned_board, player_id).is_err() {
                continue; // Skip invalid actions
            }
            let score = heuristic(&cloned_board, player_id);
            if score > best_score {
                best_score = score;
                best_action.clear();
                best_action.push(action);
            } else if score == best_score {
                best_action.push(action);
            }
        }

        if let Some(action) = best_action.into_iter().choose(&mut rand::rng()) {
            return action.execute(board, player_id);
        } else {
            // If no valid action found, fallback to simple strategy
            SimpleStrategy::make_move(player_id, board)
        }
    }
}

impl HardStrategy {
    /// search for the best flow operation
    pub fn make_move(player_id: u8, board: &mut Board) -> Result<(), OperationError> {
        MediumStrategy::make_move(player_id, board)
    }
}

// ========================
// HEURISTIC FUNCTION
// ========================

fn heuristic(board: &Board, player_id: u8) -> f64 {
    let size = board.size();
    let mut player_strength = HashMap::new();
    let mut my_strength = 0.0;

    // First stage: Calculate player strength
    let anchor_lock_map = get_anchor_lock_state(board);

    for x in 0..size {
        for y in 0..size {
            let cell_state = board.get(x, y);

            let min_moves_to_boundary =
                calculate_min_moves_to_boundary(x, y, size, &anchor_lock_map);

            if let CellState::Occupied(id) | CellState::Anchored(id) = cell_state {
                let safety = if matches!(cell_state, CellState::Anchored(_)) {
                    min_moves_to_boundary * 2.0
                } else {
                    min_moves_to_boundary
                };

                *player_strength.entry(id).or_insert(0.0) += safety;

                if id == player_id {
                    my_strength += safety;
                }
            }
        }
    }

    let balance_score = calculate_balance_score(&player_strength, player_id);

    let diversity_bonus = player_strength.len().max(1) as f64;

    let score = my_strength + balance_score * diversity_bonus;

    if my_strength == 0.0 {
        return f64::NEG_INFINITY; // If the player has no strength, return negative infinity
    } else {
        score
    }
}

fn calculate_min_moves_to_boundary(
    x: u8,
    y: u8,
    size: u8,
    lock_map: &(HashSet<u8>, HashSet<u8>, HashSet<(u8, u8)>),
) -> f64 {
    if lock_map.2.contains(&(x, y)) {
        // If the cell is locked, return the moves of a center cell
        return size as f64 / 2.0;
    }

    let left_distance = x as f64;
    let right_distance = (size - 1 - x) as f64;
    let top_distance = y as f64;
    let bottom_distance = (size - 1 - y) as f64;

    // Calculate the minimum distance to the boundary
    let row_moves = if lock_map.0.contains(&y) {
        f64::INFINITY
    } else {
        left_distance.min(right_distance)
    };

    let col_moves = if lock_map.1.contains(&x) {
        f64::INFINITY
    } else {
        top_distance.min(bottom_distance)
    };

    row_moves.min(col_moves)
}

// Calculate the balance score of the player based on the distance to the boundary
fn calculate_balance_score(player_strength: &HashMap<u8, f64>, player_id: u8) -> f64 {
    // not include itself
    let other_strength: Vec<f64> = player_strength
        .iter()
        .filter(|(id, _)| **id != player_id)
        .map(|(_, &strength)| strength)
        .collect();

    if other_strength.is_empty() {
        return 0.0; // No other players, balance score is zero
    }

    let mean = other_strength.iter().sum::<f64>() / other_strength.len() as f64;
    let var = other_strength
        .iter()
        .map(|&strength| (strength - mean).powi(2))
        .sum::<f64>()
        / other_strength.len() as f64;

    15.0 / (1.0 + var.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;
    use underflow_core::{Board, CellState};

    fn create_test_board(player_id: u8, size: u8) -> Board {
        let mut board = Board::new(size);

        // Player anchors
        if player_id == 1 {
            board.set(0, 0, CellState::Anchored(1));
        }

        // Neutral and empty cells
        board.set(2, 2, CellState::Neutral);
        board.set(2, 3, CellState::Empty);

        board
    }
}
