use underflow_core::protocol::{FlowError, *};
use underflow_core::server::*;
use underflow_core::*;

#[derive(Debug)]
pub enum OperationError {
    NoValidMove,
    ServerError(FlowError),
}

// ========================
// COMMAND HANDLING
// ========================

pub fn handle_command(
    server: &mut FlowServer,
    player_id: u8,
    cmd: FlowCommand,
) -> Result<(), OperationError> {
    server
        .handle(cmd)
        .map_err(|e| OperationError::ServerError(e))
}

pub fn try_handle_command(
    server: &FlowServer,
    player_id: u8,
    cmd: FlowCommand,
) -> Result<FlowServer, OperationError> {
    let mut server_clone = server.clone();
    server_clone
        .handle(cmd)
        .map(|_| server_clone)
        .map_err(|e| OperationError::ServerError(e))
}

// ========================
// COMMAND GENERATION
// ========================

/// Get all valid commands for a player based on the current game phase
pub fn get_valid_commands(server: &FlowServer, player_id: u8) -> Vec<FlowCommand> {
    match server.phase {
        GamePhase::Filling => get_filling_commands(server, player_id),
        GamePhase::Flowing => get_flowing_commands(server, player_id),
    }
}

/// Get all filling commands for a player during the filling phase
fn get_filling_commands(server: &FlowServer, player_id: u8) -> Vec<FlowCommand> {
    let size = server.board.size();
    (0..size)
        .flat_map(|x| (0..size).map(move |y| (x, y)))
        .filter(|&(x, y)| server.board.get(x, y) == CellState::Empty)
        .map(|(x, y)| FlowCommand::SetOccupied {
            player: player_id,
            x,
            y,
        })
        .collect()
}

/// 流动阶段可用的命令
fn get_flowing_commands(server: &FlowServer, player_id: u8) -> Vec<FlowCommand> {
    let mut commands = Vec::new();
    let size = server.board.size();

    // 添加流动命令
    for i in 0..size {
        if server.board.can_flow_x(i) {
            commands.push(FlowCommand::FlowX {
                player: player_id,
                y: i,
                positive: true,
            });
            commands.push(FlowCommand::FlowX {
                player: player_id,
                y: i,
                positive: false,
            });
        }

        if server.board.can_flow_y(i) {
            commands.push(FlowCommand::FlowY {
                player: player_id,
                x: i,
                positive: true,
            });
            commands.push(FlowCommand::FlowY {
                player: player_id,
                x: i,
                positive: false,
            });
        }
    }

    // 添加锚点操作命令
    let anchor_positions = get_valid_anchor_positions(&server.board);
    for (x, y) in anchor_positions {
        commands.push(FlowCommand::SetAnchor {
            player: player_id,
            x,
            y,
        });
    }

    commands
}

/// 获取所有有效的锚点位置
pub fn get_valid_anchor_positions(board: &Board) -> Vec<(u8, u8)> {
    let size = board.size();
    (0..size)
        .flat_map(|x| (0..size).map(move |y| (x, y)))
        .filter(|&(x, y)| matches!(board.get(x, y), CellState::Empty | CellState::Neutral))
        .collect()
}

// ========================
// UTILITY FUNCTIONS
// ========================

pub fn evaluate_filling_position(size: i32,x: i32,y: i32) -> i32 {
    let left_distance = x;
    let right_distance = size - 1 - x;
    let top_distance = y;
    let bottom_distance = size - 1 - y;
    
    let left = left_distance.min(right_distance);
    let top = top_distance.min(bottom_distance);

    left.min(top) as i32
}