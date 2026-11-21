use weiqi::{JBoard, Board, Color, GoError};

#[test]
fn readme() {
    // create a 19x19 board in the japanese rules
    if let Ok(mut board) = JBoard::init(19) {
        let mut turn = Color::Black;
        // play a black stone at 3, 3 (one indexed)
        match board.try_move(turn, (3, 3)) {
            Ok(t) => turn = t,
            Err(e) => println!("Illegal move of type: {:?}", e),
        }
    }
}