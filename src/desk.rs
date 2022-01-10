#![allow(unused)]
use crate::constants::{
    BOTTOM, BOTTOM_LEFT, BOTTOM_RIGHT, FIELD_SIZE, LIMIT_TO_DRAW_MIDDLE_LINE, MID, MID_LEFT,
    MID_RIGHT, ROW_LEN, TOP, TOP_LEFT, TOP_RIGHT, VERTICAL, VINS,
};
use crate::game::PlayerMark;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum FieldState {
    Cross,
    Zero,
    Empty,
}

pub struct Desk {
    pub fields: [FieldState; FIELD_SIZE],
}

impl Desk {
    pub fn new() -> Self {
        Self {
            fields: [FieldState::Empty; FIELD_SIZE],
        }
    }

    pub fn is_win(&self, current_player: &PlayerMark) -> bool {
        for win_case in &VINS {
            if &self.fields[win_case[0]] == current_player
                && &self.fields[win_case[1]] == current_player
                && &self.fields[win_case[2]] == current_player
            {
                return true;
            }
        }
        false
    }

    pub fn accept_turn(&mut self, row: usize, col: usize, mark: &PlayerMark) {
        match mark {
            PlayerMark::Cross => {
                self.fields[Desk::calc_pos_on_field_by_coord(row, col)] = FieldState::Cross
            }
            PlayerMark::Zero => {
                self.fields[Desk::calc_pos_on_field_by_coord(row, col)] = FieldState::Zero
            }
        }
    }

    pub fn is_cell_empty(&self, coord: (usize, usize)) -> bool {
        self.fields[Desk::calc_pos_on_field_by_coord(coord.0, coord.1)] == FieldState::Zero
    }

    pub fn is_all_marked(&self) -> bool {
        for i in &self.fields {
            if i == &FieldState::Empty {
                return false;
            }
        }
        true
    }

    pub fn draw(&self) {
        Desk::draw_horizontal_line(TOP_LEFT, TOP_RIGHT, TOP);
        for i in 0..self.fields.len() {
            if self.fields[i] == FieldState::Empty {
                Desk::draw_mark_line(' ');
            } else if self.fields[i] == FieldState::Zero {
                Desk::draw_mark_line('o');
            } else if self.fields[i] == FieldState::Cross {
                Desk::draw_mark_line('x');
            }

            if (i + 1) % ROW_LEN == 0 {
                print!("{}   ", VERTICAL);
                println!("");
                if i < LIMIT_TO_DRAW_MIDDLE_LINE {
                    Desk::draw_horizontal_line(MID_LEFT, MID_RIGHT, MID);
                }
            }
        }
        Desk::draw_horizontal_line(BOTTOM_LEFT, BOTTOM_RIGHT, BOTTOM);
    }

    fn draw_horizontal_line(corner_start: char, corner_end: char, center: char) {
        let sep = "\u{2500}\u{2500}\u{2500}";
        println!(
            "{}{sep}{}{sep}{}{sep}{}",
            corner_start,
            center,
            center,
            corner_end,
            sep = sep
        );
    }

    fn draw_mark_line(mark: char) {
        print!("{} {} ", VERTICAL, mark);
    }

    fn calc_pos_on_field_by_coord(row: usize, col: usize) -> usize {
        ((row - 1) * ROW_LEN) + col - 1
    }
}

impl PartialEq<PlayerMark> for FieldState {
    fn eq(&self, other: &PlayerMark) -> bool {
        if self == &FieldState::Zero && other == &PlayerMark::Zero {
            return true;
        }

        if self == &FieldState::Cross && other == &PlayerMark::Cross {
            return true;
        }
        false
    }
}
