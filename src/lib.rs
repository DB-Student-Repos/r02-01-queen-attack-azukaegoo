#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition
}


impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        /*unimplemented!(
            "Construct a ChessPosition struct, given the following rank, file: ({rank}, {file}). If the position is invalid return None."
        ); */
        match (rank, file) {
            (0..=7, 0..=7) => Some(ChessPosition { rank, file }),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
       /* unimplemented!("Given the chess position {position:?}, construct a Queen struct.");*/
        Queen {
            position
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        /* unimplemented!("Determine if this Queen can attack the other Queen {other:?}");*/
        let player_one = &self.position;
        let player_two = &other.position;  // they both account for rows and column position

        let horizontal = (player_one.rank - player_two.rank).abs();
        let vertical = (player_one.file - player_two.file).abs();     // they both account for diagonal position


        player_one.rank == player_two.rank || player_one.file == player_two.file || horizontal == vertical
    }
}
