#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_cluster() {
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
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    Black,
    White
}

// a different board structure is used for all the rulesets (AGA, Japanese, and Chinese) so that their trait implementations to be unique
pub struct JBoard {
    size: usize,
    stones: Vec<Option<Color>>,
    white_captures: usize,
    black_captures: usize,
    turn: Color,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GoError {
    InvalidBoardSize,
    SelfCapture,
    OverlappingStones,
    OutOfBoard,
    NoStoneFound,
}



fn check_for_overlap(stones: &Vec<Option<Color>>, place: (usize, usize), size: usize) -> bool {
    if stones[place.0 + place.1 * size].is_none() {
        false
    } else {
        true
    }
}

/*
// self capture is tested by playing the stone, building the cluster, checking if the 
// cluster has no liberties becuase of that move, and reversing it if it doesn't.
// its is important to check and evalutate captures before self-capture is checked
fn is_self_capture() -> bool {

}

fn is_capturing() -> bool {

}
*/

// assumes the nodes being checked is already in the set
// eval liberties checks for nones and the oposite color of the cluster
fn check_node(
    stones: &Vec<Option<Color>>, 
    set: &mut Vec<(usize, usize)>, 
    color: Color, space: (usize, usize), 
    size: usize, 
    eval_liberties: bool
) -> (usize, bool) {
    let mut count = 0;
    let mut border = 0;
    // to the right
    if space.0 < size - 1{
        if !set.contains(&(space.0 + 1, space.1)) {
            let checking_node = stones[(space.0 + 1) + space.1 * size];
            if !eval_liberties {
                if checking_node == Some(color) {
                    border += 1;
                    set.push((space.0 + 1, space.1));
                    count += 1;
                }
            } else {
                if checking_node.is_none() || checking_node.unwrap() != color {
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
            if !eval_liberties {
                if checking_node == Some(color) {
                    border += 1;
                    set.push((space.0 - 1, space.1));
                    count += 1;
                }
            } else {
                if checking_node.is_none() || checking_node.unwrap() != color {
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
            if !eval_liberties {
                if checking_node == Some(color) {
                    border += 1;
                    set.push((space.0, space.1 - 1));
                    count += 1;
                }
            } else {
                if checking_node.is_none() || checking_node.unwrap() != color {
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
            if !eval_liberties {
                if checking_node == Some(color) {
                    border += 1;
                    set.push((space.0, space.1 + 1));
                    count += 1;
                }
            } else {
                if checking_node.is_none() || checking_node.unwrap() != color {
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
        let (lv, b) = check_node(stones, &mut cluster_stones, color, space, size, false);
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
                let (lv, b) = check_node(stones, &mut cluster_stones, color, node, size, false);
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
            check_node(stones, &mut liberties, color, stone, size, true);
        }
        Ok((color, cluster_stones, liberties))
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

/* 
impl Board for JBoard {
    fn init(size: usize) -> Result<Self, GoError> {
        if size != 9 && size != 13 && size != 19 {
            Err(GoError::InvalidBoardSize)
        } else {
            let mut stones: Vec<Option<Color>> = Vec::with_capacity(size * size);
            for y in 0..size {
                for x in 0..size {
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
*/