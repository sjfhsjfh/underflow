use std::collections::{HashMap, HashSet};
use underflow_core::{Board, CellState, server::FlowServer};

// ========================
// HEURISTIC FUNCTION
// ========================

pub fn heuristic(server: &FlowServer, player_id: u8) -> f64 {
    let board = &server.board;
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

pub fn calculate_min_moves_to_boundary(
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
        size as f64 / 2.0
    } else {
        left_distance.min(right_distance)
    };

    let col_moves = if lock_map.1.contains(&x) {
        size as f64 / 2.0
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

    if other_strength.len() == 0 || other_strength.len() == 1 {
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
