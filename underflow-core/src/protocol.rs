pub enum FlowCommand {
    SetOccupied { player: u8, x: u8, y: u8 },
    SetAnchor { player: u8, x: u8, y: u8 },
    FlowX { player: u8, y: u8, positive: bool },
    FlowY { player: u8, x: u8, positive: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GamePhase {
    Filling,
    Flowing,
}

impl GamePhase {
    pub fn is_filling(&self) -> bool {
        matches!(self, Self::Filling)
    }

    pub fn is_flowing(&self) -> bool {
        matches!(self, Self::Flowing)
    }
}

#[derive(Debug)]
pub enum FlowError {
    /// e.g. SetOccupied during Flowing phase
    InvalidPhase,

    /// Flow operation blocked
    BlockedByAnchor,

    /// Shouldn't appear in normal game process
    IndexOutOfRange,

    /// Not your turn
    NotYourTurn,

    /// Not changing the current board state or changing it into a previously shown state.
    Recurrence,

    /// Trying to set anchor or fill in a cell that is already occupied
    AlreadyOccupied,
}

impl std::fmt::Display for FlowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlowError::AlreadyOccupied => write!(f, "Already occupied")?,
            FlowError::BlockedByAnchor => write!(f, "Blocked by anchor")?,
            FlowError::IndexOutOfRange => write!(f, "Index out of range")?,
            FlowError::InvalidPhase => write!(f, "Invalid phase")?,
            FlowError::NotYourTurn => write!(f, "Not your turn")?,
            FlowError::Recurrence => write!(f, "Recurrence")?,
        }
        Ok(())
    }
}

impl std::error::Error for FlowError {}

pub type FlowResponse = std::result::Result<(), FlowError>;
