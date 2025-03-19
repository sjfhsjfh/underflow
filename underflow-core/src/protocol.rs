use crate::board::CellState;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[allow(dead_code)]
const SERVER_ID: u8 = u8::MAX;

/// An enum that describes the types of commands the server handles.
/// Note that the command does not take into account that the sender of the command might be invalid.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum CommandContent {
    FlowX(u8, bool),
    FlowY(u8, bool),
    Put(CellState, u8, u8),
    Exchange(u8, u8, u8, u8, u8),
}

/// An enum that the types of responses the server returns.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum ResponseContent {
    /// Indicates that game is over. Announces winner in field 0.
    GameOver(u8),
    /// Indicates that phase has changed. Appoints who to play next in field 0.
    PhaseChange(u8),
    /// Indicates that the play is valid and does not have any other effect. 
    /// Also appoints who to play next in field 0.
    Valid(u8),
    /// Indicates that one player has been eliminated.
    /// The id of eliminated player stores in field 0, and the id to play next in 1.
    Elimination(u8, u8),
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub enum UnderflowError {
    // This category is general error.
    IndexOutOfBounds,
    InvalidPlayerId,
    // This category is about errors on flowing.
    BlockedByAnchor,
    // This is about errors on placing blocks.
    AlreadyOccupied,
    AlreadyPlacedAnchor,
    // And this is about errors on changing the place of anchor.
    InvalidChangeSource,
    InvalidChangeDestination,
}

impl Display for UnderflowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

impl Error for UnderflowError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<T> {
    from: u8,
    to: u8,
    content: T,
}