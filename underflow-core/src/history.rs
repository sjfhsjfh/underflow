use crate::Board;

pub struct BoardHistory {
    data: Vec<Vec<Board>>,
}

impl BoardHistory {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn is_recurrence(&self, board: &Board) -> bool {
        if let Some(stat) = board.stat() {
            if let Some(collection) = self.data.get(stat.total_unoccupied) {
                return collection.contains(board);
            }
        }
        false
    }

    /// Not checked, use [BoardHistory::is_recurrence] first
    pub fn push(&mut self, board: &Board) {
        let unoccupied = board.stat().unwrap().total_unoccupied;
        while self.data.len() <= unoccupied {
            self.data.push(Vec::new());
        }
        self.data[unoccupied].push(board.clone());
    }
}

// TODO: tests
