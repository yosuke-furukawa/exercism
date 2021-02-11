#[derive(Debug)]
pub struct ChessPosition{
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank >= 8 || file < 0 || file >= 8 {
            return None;
        }
        Some(ChessPosition{rank, file})
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen{ position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let rank1 = self.position.rank;
        let file1 = self.position.file;
        let rank2 = other.position.rank;
        let file2 = other.position.file;
        if rank1 == rank2 || file1 == file2 {
            return true;
        }
        for d in 1..7 {
            let x1 = (rank1 + d + 8) % 8;
            let y1 = (file1 + d + 8) % 8;
            let x2 = (rank1 - d + 8) % 8;
            let y2 = (file1 - d + 8) % 8;
            match (rank2, file2) {
                (r2, f2) if r2 == x1 && f2 == y1 => {
                    return true;
                },
                (r2, f2) if r2 == x2 && f2 == y2 => {
                    return true;
                },
                (r2, f2) if r2 == x1 && f2 == y2 => {
                    return true;
                },
                (r2, f2) if r2 == x2 && f2 == y1 => {
                    return true;
                }
                _ => continue
            }
        }
        false
    }
}
