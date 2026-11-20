#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    Black,
    White
}

// a different board structure is used for all the rulesets (AGA, Japanese, and Chinese)
pub struct JBoard {
    size: usize,
    stones: Vec<Option<Color>>,
    white_captures: usize,
    black_captures: usize,
    turn: Color,
}

pub enum GoError {
    InvalidBoardSize,
    SelfCapture,
    OverlappingStones,
    OutOfBoard,
    NoStoneFound,
}

fn check_for_overlap() -> bool {

}

fn is_self_capture() -> bool {

}

fn is_capturing() -> bool {

}

// asumes the nodes being checked is already in the set
// eval liberties checks for nones and the oposite color of the cluster
fn check_node(
    stones: &Vec<Option<Color>>, 
    set: &mut Vec<(usize, usize)>, 
    color: Color, space: (usize, usize), 
    size: usize, 
    eval_liberties: bool
) -> usize {
    let mut count = 0;
    // to the right
    if !set.contains(&(space.0 + 1, space.1)) {
        if space.0 + 1 <= size && space.1 <= size {
            let checking_node = stones[(space.0 + 1) * size + space.1];
            if !eval_liberties {
                if checking_node == Some(color) {
                    set.push((space.0 + 1, space.1));
                    count += 1;
                }
            } else {
                if checking_node.is_none() || checking_node.unwrap() != color {
                    set.push((space.0 + 1, space.1));
                    count += 1;
                }
            }
        }
    }
    // to the left
    if !set.contains(&(space.0 - 1, space.1)) {
        if space.0 + 1 <= size && space.1 <= size {
            let checking_node = stones[(space.0 - 1) * size + space.1];
            if !eval_liberties {
                if checking_node == Some(color) {
                    set.push((space.0 - 1, space.1));
                    count += 1;
                }
            } else {
                if checking_node.is_none() || checking_node.unwrap() != color {
                    set.push((space.0 - 1, space.1));
                    count += 1;
                }
            }
        }
    }
    // up
    if !set.contains(&(space.0 , space.1 + 1)) {
        if space.0 + 1 <= size && space.1 <= size {
            let checking_node = stones[space.0 * size + (space.1 + 1)];
            if !eval_liberties {
                if checking_node == Some(color) {
                    set.push((space.0, space.1 + 1));
                    count += 1;
                }
            } else {
                if checking_node.is_none() || checking_node.unwrap() != color {
                    set.push((space.0, space.1 + 1));
                    count += 1;
                }
            }
        }
    }
    // down
    if !set.contains(&(space.0 , space.1 - 1)) {
        if space.0 + 1 <= size && space.1 <= size {
            let checking_node = stones[space.0 * size + (space.1 - 1)];
            if !eval_liberties {
                if checking_node == Some(color) {
                    set.push((space.0, space.1 - 1));
                    count += 1;
                }
            } else {
                if checking_node.is_none() || checking_node.unwrap() != color {
                    set.push((space.0, space.1 - 1));
                    count += 1;
                }
            }
        }
    }
    count
}

fn build_cluster(stones: &Vec<Option<Color>>, space: (usize, usize), size: usize) -> Result<(Color, Vec<(usize, usize)>), GoError> {
    if !stones[space.0 * size + space.1].is_none() {
        // cluster stones is the entire blob
        // edge is the nodes we are checking this turn
        // last_visited is the number of nodes in the edge

        // we loop over the edge, but pass the entire blob to check because it needs to never visit twice
        let color = stones[space.0 * size + space.1].unwrap();
        let mut cluster_stones: Vec<(usize, usize)> = Vec::new();
        let mut last_visited: usize = 1;
        let edge: Vec<(usize, usize)> = Vec::from([
            (space.0 + 1, space.1), 
            (space.0 - 1, space.1), 
            (space.0, space.1 + 1), 
            (space.0, space.1 - 1),
        ]);

        cluster_stones.push(space);
        while last_visited > 0 {
            let mut count = 0;
            for node in &edge {
                count += check_node(stones, &mut cluster_stones, color, *node, size, false);
            }
            last_visited = count;
        }

        Ok((color, cluster_stones))
    } else {
        Err(GoError::NoStoneFound)
    }
}

fn get_cluster_liberties() {

}

pub trait Board: Sized {
    fn check_move(&self, color: Color, space: (usize, usize)) -> bool;
    fn init(size: usize) -> Result<Self, GoError>;
    fn try_move(&mut self, color: Color, space: (usize, usize)) -> Result<(), GoError>;
}

impl Board for JBoard {
    fn init(size: usize) -> Result<Self, GoError> {
        if size != 9 && size != 13 && size != 19 {
            Err(GoError::InvalidBoardSize)
        } else {
            let mut stones: Vec<Option<Color>> = Vec::with_capacity(size * size);
            for x in 0..size {
                for y in 0..size {
                    stones.push(None)
                }
            }
            Ok(JBoard {
                size: size,
                stones: stones,
                white_captures: 0,
                black_captures: 0,
                turn: Color::Black,
            })
        }   
    }
    fn check_move(&self, color: Color, space: (usize, usize)) -> bool {
        let flagged = false;


    }
    fn try_move(&mut self, color: Color, space: (usize, usize)) -> Result<(), GoError> {
        
    }
}