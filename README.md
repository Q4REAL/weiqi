# Weiqi-rs
Go (also known as weiqi in china or baduk in korea) is a turn based tactical board game played on a 19x19, 13x13, or 9x9 board. It is played by placing stones on the board to surround and capture the opponent's stones and surround the most territory. This crate is a rules engine for Go. 

## Supported rulesets and features
- [ ] Complete Japanese Rules
- [ ] Complete Chinese Rules
- [ ] Complete AGA Rules

- [X] Board State Management
- [X] Capturing
- [X] General Illegal Move Detection
- [x] Ko
- [ ] SuperKo
- [ ] Go Text Protocol Support
- [ ] Game Tree and Undo
- [ ] Smart Game Format
- [ ] Edge Case Rules
- [ ] Scoring

## Usage
```Rust
use weiqi::{JBoard, Board, Color};

// create a 19x19 board in the japanese rules
let mut board = JBoard::init(19);
let mut turn = Color::Black;
// play a black stone at 3, 3 (one indexed, 0,0 at top left)
if let Err(e) = board.try_move(turn, Some((3, 3))) {
    println!("Illegal move of type: {:?}", e);
}
// pass turn
turn = Color::White;
```
## License
MIT