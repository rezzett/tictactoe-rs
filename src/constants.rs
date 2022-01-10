pub const ROW_LEN: usize = 3;
pub const FIELD_SIZE: usize = 9;
pub const LIMIT_TO_DRAW_MIDDLE_LINE: usize = 6;

pub const TOP_LEFT: char = '\u{250c}';
pub const TOP_RIGHT: char = '\u{2510}';
pub const TOP: char = '\u{252c}';
pub const MID_LEFT: char = '\u{251c}';
pub const MID_RIGHT: char = '\u{2524}';
pub const MID: char = '\u{253c}';
pub const BOTTOM_LEFT: char = '\u{2514}';
pub const BOTTOM_RIGHT: char = '\u{2518}';
pub const BOTTOM: char = '\u{2534}';
pub const VERTICAL: char = '\u{2502}';

pub const INPUT_ERR: &str = "Error: Input failed at Game::ask_player_coord";
pub const INVALID_INPUT: &str = "Your position is invalid.Please try again.";
pub const INPUT_OUT_OF_RANGE_MSG: &str = "Your position is out of field.Please try again.";
pub const NOT_FREE_POSITION_MSG: &str = "Your position isn't free.Please enter another position.";

pub const VINS: [[usize; 3]; 8] = [
    [0, 1, 2], //  0 1 2
    //  3 4 5
    //  6 7 8
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];
