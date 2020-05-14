#[derive(Debug)]
pub struct ChessPosition {
    pub rank: i32,
    pub file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pub position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if Self::is_ok(rank) && Self::is_ok(file) {
            Some(Self { rank, file })
        } else {
            None
        }
    }

    fn is_ok(coord: i32) -> bool {
        coord >= 0 && coord < 8
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        match (
            self.position.rank - other.position.rank,
            self.position.file - other.position.file,
        ) {
            (0, _) => true,
            (_, 0) => true,
            (x, y) => x == y || x == -y,
        }
    }
}
