use weiqi::{JBoard, Board, Color, GoError};

#[test]
fn readme() {
    // create a 19x19 board in the japanese rules
    let mut board = JBoard::init(19);
    let mut turn = Color::Black;
    // play a black stone at 3, 3 (one indexed)
    if let Err(e) = board.try_move(turn, Some((3, 3))) {
        println!("Illegal move of type: {:?}", e);
    }
    // pass turn
    turn = Color::White;
}

#[test]
fn ko() {
    /*
       1  2  3  4  
    1 [ ][B][W][ ]
    2 [B][W][ ][W]
    3 [ ][B][W][ ]
    4 [ ][ ][ ][ ]
     */
    let mut board = JBoard::init(4);

    println!("{:?}", board.try_move(Color::Black, Some((2, 1))));
    println!("{:?}", board.try_move(Color::Black, Some((1, 2))));
    println!("{:?}", board.try_move(Color::Black, Some((2, 3))));
    println!("{:?}", board.try_move(Color::Black, Some((3, 2))));

    println!("{:?}", board.try_move(Color::White, Some((3, 1))));
    println!("{:?}", board.try_move(Color::White, Some((3, 3))));
    println!("{:?}", board.try_move(Color::White, Some((4, 2))));

    println!("{:?}", board.try_move(Color::White, Some((2, 2))));

    assert_eq!(Err(GoError::Ko), board.try_move(Color::Black, Some((3, 2))))
}