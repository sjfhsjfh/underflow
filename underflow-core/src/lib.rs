use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CellState {
    Empty,
    Neutral,
    Occupied(u8),
    Anchored(u8),
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FlowError {
    BlockedByAnchor,
    OutOfBounds,
}

impl Display for FlowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FlowError::BlockedByAnchor => write!(f, "Blocked by anchor")?,
            FlowError::OutOfBounds => write!(f, "Argument out of bounds")?,
        }
        Ok(())
    }
}

impl Error for FlowError {}

impl CellState {
    pub fn is_anchor(&self) -> bool {
        matches!(self, CellState::Anchored(_))
    }

    /// Check if the cell is occupied, return the player id if true
    pub fn occupied_then_id(&self) -> Option<u8> {
        match self {
            CellState::Occupied(player) => Some(*player),
            _ => None,
        }
    }
}

pub enum BoardStat {
    NotReady,
    Ready {
        /// Player id -> occupied cell count
        player_stat: HashMap<u8, u8>,

        /// This may be useful for history indexing since its monotonic decreasing
        total_occupied: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// # The Game Board
///
/// |     Axis     |  Origin  |  Index  |
/// |--------------|----------|---------|
/// | +x: →, +y: ↓ | Top Left | 0-based |
pub struct Board {
    cells: Vec<Vec<CellState>>,
    size: u8,
}

impl Board {
    pub fn new(size: u8) -> Self {
        let cells = vec![vec![CellState::Empty; size as usize]; size as usize];
        Self { cells, size }
    }

    /// No size check
    pub fn get(&self, x: u8, y: u8) -> CellState {
        self.cells[x as usize][y as usize]
    }

    /// No size check
    pub fn set(&mut self, x: u8, y: u8, state: CellState) {
        self.cells[x as usize][y as usize] = state;
    }

    #[inline]
    /// Check if all the cells in this row are not anchored, no size check
    pub fn can_flow_x(&self, y: u8) -> bool {
        !(0..self.size).any(|x| self.get(x, y).is_anchor())
    }

    #[inline]
    /// Check if all the cells in this column are not anchored, no size check
    pub fn can_flow_y(&self, x: u8) -> bool {
        !(0..self.size).any(|y| self.get(x, y).is_anchor())
    }

    /// Flow the cells in the x-axis, if this row is anchored, or the row is out of bound, return `Err`
    pub fn flow_x(&mut self, y: u8, positive: bool) -> Result<(), FlowError> {
        if !self.can_flow_x(y) {
            return Err(FlowError::BlockedByAnchor);
        }
        if y >= self.size {
            return Err(FlowError::OutOfBounds);
        }
        if positive {
            for x in (0..self.size).rev() {
                if x == 0 {
                    self.set(x, y, CellState::Neutral);
                } else {
                    let state = self.get(x - 1, y);
                    self.set(x, y, state);
                }
            }
        } else {
            for x in 0..self.size {
                if x == self.size - 1 {
                    self.set(x, y, CellState::Neutral);
                } else {
                    let state = self.get(x + 1, y);
                    self.set(x, y, state);
                }
            }
        }
        Ok(())
    }

    /// Flow the cells in the y-axis, if this column is anchored, or the column is out of bound, return `Err`
    pub fn flow_y(&mut self, x: u8, positive: bool) -> Result<(), FlowError> {
        if !self.can_flow_y(x) {
            return Err(FlowError::BlockedByAnchor);
        }
        if x >= self.size {
            return Err(FlowError::OutOfBounds);
        }
        if positive {
            for y in (0..self.size).rev() {
                if y == 0 {
                    self.set(x, y, CellState::Neutral);
                } else {
                    let state = self.get(x, y - 1);
                    self.set(x, y, state);
                }
            }
        } else {
            for y in 0..self.size {
                if y == self.size - 1 {
                    self.set(x, y, CellState::Neutral);
                } else {
                    let state = self.get(x, y + 1);
                    self.set(x, y, state);
                }
            }
        }
        Ok(())
    }

    /// Fill the board to make the empty cell count is a multiple of player count
    pub fn init(player_count: u8, size: u8) -> Self {
        let mut board = Self::new(size);
        // TODO: currently only 2-4 players are supported
        match player_count {
            2 | 4 => {
                if size % 2 == 0 {
                    return board;
                }
                let center = size / 2;
                board.set(center, center, CellState::Neutral);
                board
            }
            3 => {
                if size % 3 == 0 {
                    return board;
                }
                board.set(0, 0, CellState::Neutral);
                board.set(0, size - 1, CellState::Neutral);
                board.set(size - 1, 0, CellState::Neutral);
                board.set(size - 1, size - 1, CellState::Neutral);
                board
            }
            _ => panic!("Unsupported player count"),
        }
    }

    pub fn size(&self) -> u8 {
        self.size
    }

    pub fn stat(&self) -> BoardStat {
        if !self.is_ready() {
            return BoardStat::NotReady;
        }
        let mut player_stat = HashMap::new();
        let mut total_occupied = 0;
        self.cells.iter().flatten().for_each(|&cell| {
            if let Some(player) = cell.occupied_then_id() {
                let stat = player_stat.entry(player).or_insert(0);
                *stat += 1;
                total_occupied += 1;
            }
        });
        BoardStat::Ready {
            player_stat,
            total_occupied,
        }
    }

    /// Check if the board is ready to start the game, i.e. no empty cell
    pub fn is_ready(&self) -> bool {
        self.cells
            .iter()
            .flatten()
            .all(|&cell| cell != CellState::Empty)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                match cell {
                    CellState::Empty => write!(f, "█  ")?,
                    CellState::Neutral => write!(f, "N  ")?,
                    CellState::Occupied(id) => write!(f, "O{} ", id)?,
                    CellState::Anchored(id) => write!(f, "A{} ", id)?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_board() {
        let mut board = Board::init(3, 7);
        assert_eq!(board.size(), 7);
        assert_eq!(board.get(0, 0), CellState::Neutral);
        assert_eq!(board.get(0, 6), CellState::Neutral);
        assert_eq!(board.get(6, 0), CellState::Neutral);
        assert_eq!(board.get(6, 6), CellState::Neutral);
        assert_eq!(board.get(3, 3), CellState::Empty);
        board.set(3, 3, CellState::Occupied(1));
        assert_eq!(board.get(3, 3), CellState::Occupied(1));
        assert!(board.can_flow_x(3));
        assert!(board.can_flow_y(3));
        assert_eq!(board.flow_x(3, true), Ok(()));
        assert_eq!(board.get(3, 3), CellState::Empty);
        assert_eq!(board.get(4, 3), CellState::Occupied(1));
        assert_eq!(board.flow_y(4, true), Ok(()));
        assert_eq!(board.flow_y(4, false), Ok(()));
        assert_eq!(board.get(4, 3), CellState::Occupied(1));
        board.set(4, 3, CellState::Anchored(1));
        assert!(!board.can_flow_x(3));
        assert!(!board.can_flow_y(4));
        assert!(!board.is_ready());
        let produced = format!("{}", board)
            .lines()
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let expected = [
            "N  █  █  N  █  █  N",
            "█  █  █  █  █  █  █",
            "█  █  █  █  █  █  █",
            "█  █  █  █  █  █  █",
            "█  █  █  A1 █  █  N",
            "█  █  █  █  █  █  █",
            "N  █  █  █  █  █  N",
        ]
        .join("\n");
        assert_eq!(produced, expected);
    }
}
