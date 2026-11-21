//! # Weiqi-rs
//! Go (also known as weiqi in china or baduk in korea) is a turn based tactical board game played on a 19x19, 13x13, or 9x9 board. It is played by placing stones on the board to surround and capture the opponent's stones and surround the most territory. This crate is a rules engine for Go.
//! 
//! ## Features
//! - Board State Management
//! - Capturing
//! - General Illegal Move Detection
//! ## Usage
//! ```Rust
//! use weiqi::{JBoard, Board, Color, GoError};
//!
//! // create a 19x19 board in the japanese rules
//! if let Ok(mut board) = JBoard::init(19) {
//!     let mut turn = Color::Black;
//!     // play a black stone at 3, 3 (one indexed)
//!     match board.try_move(turn, (3, 3)) {
//!         Ok(t) => turn = t,
//!         Err(e) => println!("Illegal move of type: {:?}", e),
//!     }
//! }
//! ```

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cluster_test() {

        // checks right -> left -> up -> down
        /*
           0  1  2
        0 [ ][B][ ]
        1 [ ][B][ ]
        2 [W][W][W]
         */
        let stones: Vec<Option<Color>> = Vec::from([
            None,               Some(Color::Black), None,
            None,               Some(Color::Black), None,
            Some(Color::White), Some(Color::White), Some(Color::White)
        ]);
        assert_eq!(Ok((Color::Black, Vec::from([(1, 1), (1, 0)]), Vec::from([(2, 1), (0, 1), (1, 2), (2, 0), (0, 0)]))), build_cluster(&stones, (1, 1), 3));
        /*
           0  1  2  3
        0 [ ][B][B][B]
        1 [ ][B][ ][B]
        2 [W][W][B][B]
        3 [W][W][B][B]
         */
        let stones: Vec<Option<Color>> = Vec::from([
            None,               Some(Color::Black), Some(Color::Black), Some(Color::Black),
            None,               Some(Color::Black), None,               Some(Color::Black),
            Some(Color::White), Some(Color::White), Some(Color::Black), Some(Color::Black),
            Some(Color::White), Some(Color::White), Some(Color::Black), Some(Color::Black),
        ]);
        assert_eq!(Ok((Color::Black, Vec::from([(1, 1), (1,0),(2,0),(3,0),(3, 1),(3, 2),(2,2),(3,3), (2,3)]), Vec::from([(2, 1), (0, 1), (1, 2), (0, 0), (1, 3)]))), build_cluster(&stones, (1, 1), 4));

        /*
           0  1  2
        0 [ ][B][ ]
        1 [B][B][B]
        2 [ ][B][ ]
         */
        let stones: Vec<Option<Color>> = Vec::from([
            None,               Some(Color::Black), None,
            Some(Color::Black), Some(Color::Black), Some(Color::Black),
            None,               Some(Color::Black), None,
        ]);
        assert_eq!(Ok((Color::Black, Vec::from([(2, 1), (1, 1), (0, 1), (1, 0), (1, 2)]), Vec::from([(2, 0), (2, 2), (0, 0), (0, 2)]))), build_cluster(&stones, (2, 1), 3));
    }
    #[test]
    fn capture_test() {
        /*
           0  1  2  
        0 [B][B][B]
        1 [B][W][ ]
        2 [B][B][B]
        */
        let stones: Vec<Option<Color>> = Vec::from([
            Some(Color::Black), Some(Color::Black),  Some(Color::Black), 
            Some(Color::Black), Some(Color::White), None,
            Some(Color::Black), Some(Color::Black), Some(Color::Black), 
        ]);
        assert_eq!(Ok(Vec::from([(1, 1)])), check_capture(&stones, (2, 1), 3, Color::Black));

        /*
           0  1  2  3  4 
        0 [ ][B][B][B][ ]
        1 [B][B][W][B][B]
        2 [B][W][ ][W][B]
        3 [B][B][W][B][B]
        4 [ ][B][B][B][ ]
        */

        let stones: Vec<Option<Color>> = Vec::from([
            None,              Some(Color::Black), Some(Color::Black), Some(Color::Black), None,
            Some(Color::Black),Some(Color::Black), Some(Color::White), Some(Color::Black), Some(Color::Black),
            Some(Color::Black),Some(Color::White), None,               Some(Color::White), Some(Color::Black),
            Some(Color::Black),Some(Color::Black), Some(Color::White), Some(Color::Black), Some(Color::Black),
            None,              Some(Color::Black), Some(Color::Black), Some(Color::Black), None,
        ]);
        assert_eq!(Ok(Vec::from([(3, 2), (1, 2), (2, 1), (2, 3)])), check_capture(&stones, (2, 2), 5, Color::Black));
        
        /*
           0  1  2  3  4 
        0 [ ][B][B][B][ ]
        1 [B][W][W][B][B]
        2 [B][W][ ][W][B]
        3 [B][B][W][W][B]
        4 [ ][B][B][B][ ]
        */

        let stones: Vec<Option<Color>> = Vec::from([
            None,              Some(Color::Black), Some(Color::Black), Some(Color::Black), None,
            Some(Color::Black),Some(Color::White), Some(Color::White), Some(Color::Black), Some(Color::Black),
            Some(Color::Black),Some(Color::White), None,               Some(Color::White), Some(Color::Black),
            Some(Color::Black),Some(Color::Black), Some(Color::White), Some(Color::White), Some(Color::Black),
            None,              Some(Color::Black), Some(Color::Black), Some(Color::Black), None,
        ]);
        assert_eq!(Ok(Vec::from([(3, 2), (3, 3), (2, 3), (1, 2), (1, 1), (2, 1)])), check_capture(&stones, (2, 2), 5, Color::Black));
    }
    #[test]
    fn self_capture_test() {
        /*
           0  1  2
        0 [ ][W][ ]
        1 [W][ ][W]
        2 [ ][W][ ] 
         */
        let mut stones: Vec<Option<Color>> = Vec::from([
            None,               Some(Color::White), None, 
            Some(Color::White), None,               Some(Color::White),
            None,               Some(Color::White), None, 
        ]);
        assert_eq!(true, is_self_capture(&mut stones, (1, 1), 3, Color::Black))
    }
}

/// Used to represent the color of a stone or which player's turn it is
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    Black,
    White
}

// a different board structure is used for all the rulesets (AGA, Japanese, and Chinese) so that their trait implementations to be unique
/// A version of Board that implements the Japanese Rules
pub struct JBoard {
    pub size: usize,
    pub stones: Vec<Option<Color>>,
    pub white_captures: usize,
    pub black_captures: usize,
}

/// Represents all the API-level errors weiqi can throw.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GoError {
    // thrown at init
    /// Thrown when the board size given to init is not 9x9, 13x13, or 19x19
    InvalidBoardSize,
    // thrown at try_move
    /// Thrown when a requested move would result in Self Capture
    SelfCapture,
    // thrown at try_move
    /// Thrown when a requested move would result in Overlapping Stones
    OverlappingStones,
    // thrown at try_move
    /// Thrown when a requested move is outside the board boundries
    OutOfBoard,
    // thrown at build_cluster
    NoStoneFound,
    // thrown at score
    /// Thrown during scoring when a given board state is impossible
    IllegalBoard,
}



fn is_overlaping(stones: &Vec<Option<Color>>, place: (usize, usize), size: usize) -> bool {
    if stones[place.0 + place.1 * size].is_none() {
        false
    } else {
        true
    }
}


// its is important to check and evalutate captures before self-capture is checked
fn is_self_capture(stones: &mut Vec<Option<Color>>, space: (usize, usize), size: usize, color: Color) -> bool {
    let prev = stones[space.0 + space.1 * size];
    stones[space.0 + space.1 * size] = Some(color);
    if let Ok((_, _, liberties)) = build_cluster(stones, space, size) {
        let mut count = liberties.len();
        for lib in liberties {
            if stones[lib.0 * lib.1 * size] != Some(color) {
                count -= 1;
            }
        }
        if count == 0 {
            stones[space.0 + space.1 * size] = prev;
            true
        } else {
            stones[space.0 + space.1 * size] = prev;
            false
        }
    } else {
        stones[space.0 + space.1 * size] = prev;
        false
    }
}


// input takes the move that could capture, not what is being captured
fn check_capture(stones: &Vec<Option<Color>>, space: (usize, usize), size: usize, color: Color) -> Result<Vec<(usize, usize)>, GoError> {
    // adjacent nodes are the possible starting points for clusters that can be removed
    let mut adjacent_nodes: Vec<(usize, usize)> = Vec::new();
    let mut tripped = false;
    let mut captured_stones: Vec<(usize, usize)> = Vec::new();
    let mut er: Option<GoError> = None;
    let (_, _) = check_node(stones, &mut adjacent_nodes, color, space, size, NodeMode::Cap);
    for node in adjacent_nodes {
        let r = build_cluster(stones, node, size);
        match r {
            Ok((_, cluster, liberties)) => {
                // - 1 becuase the stone that captures is not simulated to be placed yet
                let mut total_l = liberties.len() - 1;
                for l in liberties {
                    if !stones[l.0 + l.1 * size].is_none() {
                        total_l -= 1;
                    }
                }
                if total_l == 0 {
                    for stone in &cluster {
                        if !captured_stones.contains(stone) {
                            captured_stones.push(*stone)
                        }
                    }
                }
            }
            Err(e) => {tripped = true; er = Some(e); break}
        }
    }
    if tripped {
        Err(er.unwrap())
    } else {
        Ok(captured_stones)
    }
}

enum NodeMode {
    // visits same colored consecutive nodes
    Cluster,
    // visits opposite colored or none nodes and determines of a node is a border node
    Liberty,
    // checks for opposite colored nodes and returns all nodes that are
    Cap,
}

// assumes the nodes being checked is already in the set
// eval liberties checks for nones and the oposite color of the cluster
fn check_node(
    stones: &Vec<Option<Color>>, 
    set: &mut Vec<(usize, usize)>, 
    color: Color, space: (usize, usize), 
    size: usize, 
    mode: NodeMode
) -> (usize, bool) {
    // count is the number of stones newly added to the set
    // border is the number of total nodes that were visted (even if they aren't new)
    let mut count = 0;
    let mut border = 0;
    // to the right
    if space.0 < size - 1{
        if !set.contains(&(space.0 + 1, space.1)) {
            let checking_node = stones[(space.0 + 1) + space.1 * size];
            match mode {
                NodeMode::Cluster => if checking_node == Some(color) {
                    border += 1;
                    set.push((space.0 + 1, space.1));
                    count += 1;
                }
                NodeMode::Liberty => if checking_node.is_none() || checking_node.unwrap() != color {
                    set.push((space.0 + 1, space.1));
                    count += 1;
                }
                NodeMode::Cap => if !checking_node.is_none() && checking_node.unwrap() != color {
                    set.push((space.0 + 1, space.1));
                    count += 1;
                }
            }
        } else {
            border += 1;
        }
    }
    // to the left
    if space.0 > 0 {
        if !set.contains(&(space.0 - 1, space.1)) {
            let checking_node = stones[(space.0 - 1) + space.1 * size];
            match mode {
                NodeMode::Cluster => if checking_node == Some(color) {
                    border += 1;
                    set.push((space.0 - 1, space.1));
                    count += 1;
                }
                NodeMode::Liberty => if checking_node.is_none() || checking_node.unwrap() != color {
                    set.push((space.0 - 1, space.1));
                    count += 1;
                }
                NodeMode::Cap => if !checking_node.is_none() && checking_node.unwrap() != color {
                    set.push((space.0 - 1, space.1));
                    count += 1;
                }
            }
        } else {
            border += 1;
        }
    }
    // up
    if space.1 > 0 {
        if !set.contains(&(space.0 , space.1 - 1)) {
            let checking_node = stones[space.0 + (space.1 - 1) * size];
            match mode {
                NodeMode::Cluster => if checking_node == Some(color) {
                    border += 1;
                    set.push((space.0, space.1 - 1));
                    count += 1;
                }
                NodeMode::Liberty => if checking_node.is_none() || checking_node.unwrap() != color {
                    set.push((space.0, space.1 - 1));
                    count += 1;
                }
                NodeMode::Cap => if !checking_node.is_none() && checking_node.unwrap() != color {
                    set.push((space.0, space.1 - 1));
                    count += 1;
                }
            }
        } else {
            border += 1;
        }
    }
    // down
    if space.1 < size - 1 {
        if !set.contains(&(space.0 , space.1 + 1)) {
            let checking_node = stones[space.0 + (space.1 + 1) * size];
            match mode {
                NodeMode::Cluster => if checking_node == Some(color) {
                    border += 1;
                    set.push((space.0, space.1 + 1));
                    count += 1;
                }
                NodeMode::Liberty => if checking_node.is_none() || checking_node.unwrap() != color {
                    set.push((space.0, space.1 + 1));
                    count += 1;
                }
                NodeMode::Cap => if !checking_node.is_none() && checking_node.unwrap() != color {
                    set.push((space.0, space.1 + 1));
                    count += 1;
                }
            }
        } else {
            border += 1;
        }
    }
    if border < 4 {
        (count, true)
    } else {
        (count, false)
    }
}

fn build_cluster(stones: &Vec<Option<Color>>, space: (usize, usize), size: usize) -> Result<(Color, Vec<(usize, usize)>, Vec<(usize, usize)>), GoError> {
    if space.0 <= size || space.1 <= size {    
        if !stones[space.0 + space.1 * size].is_none() {
            // cluster stones is the entire blob
            // edge is the nodes we are checking this turn
            // last_visited is the number of nodes in the edge

            // we loop over the edge, but pass the entire blob to check because it needs to never visit twice
            let color = stones[space.0 + space.1 * size].unwrap();
            let mut cluster_stones: Vec<(usize, usize)> = Vec::new();
            let mut last_visited: usize;
            let mut border_stones: Vec<(usize, usize)> = Vec::new();
            let mut liberties: Vec<(usize, usize)> = Vec::new();
            let mut edge: Vec<(usize, usize)> = Vec::new();
            cluster_stones.push(space);
            let (lv, b) = check_node(stones, &mut cluster_stones, color, space, size, NodeMode::Cluster);
            last_visited = lv;
            if b {
                border_stones.push(space)
            }
            for i in 0..last_visited {
                edge.push(cluster_stones[cluster_stones.len() - 1 - i]);
            }
            while last_visited > 0 {
                last_visited = 0;
                for node in edge.into_iter().rev() {
                    let (lv, b) = check_node(stones, &mut cluster_stones, color, node, size, NodeMode::Cluster);
                    last_visited += lv;
                    if b {
                        border_stones.push(node)
                    }
                }
                edge = Vec::new();
                for i in 0..last_visited {
                    edge.push(cluster_stones[cluster_stones.len() - 1 - i]);
                }
            }
            for stone in border_stones {
                check_node(stones, &mut liberties, color, stone, size, NodeMode::Liberty);
            }
            Ok((color, cluster_stones, liberties))
        } else {
            Err(GoError::NoStoneFound)
        }
    } else {
        Err(GoError::OutOfBoard)
    }
}

/// Base trait for all rulesets to inherent
pub trait Board: Sized {
    fn check_move(&mut self, color: Color, space: (usize, usize)) -> Result<(), GoError>;
    fn init(size: usize) -> Result<Self, GoError>;
    fn try_move(&mut self, color: Color, space: (usize, usize)) -> Result<(), GoError>;
    // fn score(&self) -> Result<(f32, f32), GoError>;
}


impl Board for JBoard {
    /// Builds a blank JBoard
    fn init(size: usize) -> Result<Self, GoError> {
        if size != 9 && size != 13 && size != 19 {
            Err(GoError::InvalidBoardSize)
        } else {
            let mut stones: Vec<Option<Color>> = Vec::with_capacity(size * size);
            for _ in 0..size {
                for _ in 0..size {
                    stones.push(None)
                }
            }
            Ok(JBoard {
                size: size,
                stones: stones,
                white_captures: 0,
                black_captures: 0,
            })
        }   
    }
    /// Checks if a move is legal, but does not execute the move
    fn check_move(&mut self, color: Color, space: (usize, usize)) -> Result<(), GoError> {
        let mut cs: Vec<(usize, usize)> = Vec::new();
        if !space.0 == 0 || !space.1 == 0 {
            let space = (space.0 - 1, space.1 - 1);
            if space.0 < self.size && space.1 < self.size {
                if is_overlaping(&self.stones, space, self.size) {
                    if color == Color::Black {
                        for spot in cs {
                            self.stones[spot.0 + spot.1 * self.size] = Some(Color::White);
                        }
                    } else {
                        for spot in &cs {
                            self.stones[spot.0 + spot.1 * self.size] = Some(Color::Black);
                        }
                    }
                    return Err(GoError::OverlappingStones)
                } else {
                    let cc = check_capture(&self.stones, space, self.size, color);
                    match cc {
                        Ok(rcs) => {
                            cs = rcs;
                            for spot in &cs {
                                self.stones[spot.0 + spot.1 * self.size] = None;
                            }
                            if is_self_capture(&mut self.stones, space, self.size, color) {
                                if color == Color::Black {
                                    for spot in cs {
                                        self.stones[spot.0 + spot.1 * self.size] = Some(Color::White);
                                    }
                                } else {
                                    for spot in &cs {
                                        self.stones[spot.0 + spot.1 * self.size] = Some(Color::Black);
                                    }
                                }
                                return Err(GoError::SelfCapture)
                            }
                        }
                        Err(e) => {
                            if color == Color::Black {
                                for spot in cs {
                                    self.stones[spot.0 + spot.1 * self.size] = Some(Color::White);
                                }
                            } else {
                                for spot in &cs {
                                    self.stones[spot.0 + spot.1 * self.size] = Some(Color::Black);
                                }
                            }
                            return Err(e)
                        }
                    }
                }
            } else {
                if color == Color::Black {
                    for spot in cs {
                        self.stones[spot.0 + spot.1 * self.size] = Some(Color::White);
                    }
                } else {
                    for spot in &cs {
                        self.stones[spot.0 + spot.1 * self.size] = Some(Color::Black);
                    }
                }
                return Err(GoError::OutOfBoard)
            }
        } else {
            if color == Color::Black {
                for spot in cs {
                    self.stones[spot.0 + spot.1 * self.size] = Some(Color::White);
                }
            } else {
                for spot in &cs {
                    self.stones[spot.0 + spot.1 * self.size] = Some(Color::Black);
                }
            }
            return Err(GoError::OutOfBoard)
        }
        if color == Color::Black {
            for spot in cs {
                self.stones[spot.0 + spot.1 * self.size] = Some(Color::White);
            }
        } else {
            for spot in &cs {
                self.stones[spot.0 + spot.1 * self.size] = Some(Color::Black);
            }
        }
        Ok(())
    }

    /// executes a move, and undoes if if its illegal
    fn try_move(&mut self, color: Color, space: (usize, usize)) -> Result<(), GoError> {
        if !space.0 == 0 || !space.1 == 0 {
            let space = (space.0 - 1, space.1 - 1);
            if space.0 < self.size && space.1 < self.size {
                if is_overlaping(&self.stones, space, self.size) {
                    return Err(GoError::OverlappingStones)
                } else {
                    let cc = check_capture(&self.stones, space, self.size, color);
                    match cc {
                        Ok(cs) => {
                            for spot in &cs {
                                self.stones[spot.0 + spot.1 * self.size] = None;
                                if color == Color::Black {
                                    self.black_captures += 1
                                } else {
                                    self.white_captures += 1
                                }
                            }
                            if is_self_capture(&mut self.stones, space, self.size, color) {
                                if color == Color::Black {
                                    for spot in &cs {
                                        self.stones[spot.0 + spot.1 * self.size] = Some(Color::White);
                                        self.white_captures -= 1
                                    }
                                } else {
                                    for spot in &cs {
                                        self.stones[spot.0 + spot.1 * self.size] = Some(Color::Black);
                                        self.black_captures -= 1
                                    }
                                }
                                return Err(GoError::SelfCapture)
                            } else {
                                if color == Color::Black {
                                    self.stones[space.0 + space.1 * self.size] = Some(color);
                                    Ok(())
                                } else {
                                    self.stones[space.0 + space.1 * self.size] = Some(color);
                                    Ok(())
                                }
                            }
                        }
                        Err(e) => return Err(e)
                    }
                }
            } else {
                return Err(GoError::OutOfBoard)
            }
        } else {
            return Err(GoError::OutOfBoard)
        }
    }
}