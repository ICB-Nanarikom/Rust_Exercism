#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (x, y) if 0 <= x && x <= 7 && 0 <= y && y <= 7 => Some(ChessPosition(x, y)),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        match (self.0.0, self.0.1, other.0.0, other.0.1) {
            (x1, _, x2, _) if x1 == x2 => true,
            (_, y1, _, y2) if y1 == y2 => true,
            (x1, y1, x2, y2) if x1 - y1 == x2 - y2 => true,
            (x1, y1, x2, y2) if x1 + y1 == x2 + y2 => true,
            _ => false,
        }
    }
}
