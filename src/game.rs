use crate::constants::{INPUT_ERR, INPUT_OUT_OF_RANGE_MSG, INVALID_INPUT, NOT_FREE_POSITION_MSG};
use crate::desk::{Desk, FieldState};
use rand::Rng;
use std::num::ParseIntError;

#[derive(PartialEq, Eq, Debug)]
pub enum PlayerMark {
    Cross,
    Zero,
}

pub struct Game {
    pub desk: Desk,
    pub current_player: PlayerMark,
}

impl Game {
    pub fn new() -> Self {
        Self {
            desk: Desk::new(),
            current_player: Game::get_random_first_player(),
        }
    }

    pub fn run(&mut self) {
        Game::greet();
        loop {
            println!("{} turns", &self.get_current_player_name());
            self.desk.draw();
            let coord = self.get_coord();
            self.desk
                .accept_turn(coord.0, coord.1, &self.current_player);

            if self.desk.is_win(&self.current_player) {
                self.desk.draw();
                println!("{} win!", &self.get_current_player_name());
                break;
            }

            if self.desk.is_all_marked() {
                println!("DRAW!");
                break;
            }
            self.switch_current_player();
        }
    }

    fn greet() {
        println!("******************** Tic Tac Toe ********************");
    }

    fn get_coord(&self) -> (usize, usize) {
        println!("Enter position, where you are going to draw your mark.");
        loop {
            let coord = match Game::ask_player_coord() {
                Ok((r, c)) => (r, c),
                Err(_) => {
                    println!("{}", INVALID_INPUT);
                    continue;
                }
            };

            if !Game::is_coord_valid(coord) {
                println!("{}", INPUT_OUT_OF_RANGE_MSG);
                continue;
            }

            if self.desk.is_cell_empty(coord) {
                println!("{}", NOT_FREE_POSITION_MSG);
                continue;
            }
            break coord;
        }
    }

    fn ask_player_coord() -> Result<(usize, usize), ParseIntError> {
        let mut buf = String::new();
        println!("Which row do you choose? (1-3)");
        std::io::stdin().read_line(&mut buf).expect(INPUT_ERR);
        let row = buf.trim().parse::<usize>()?;
        println!("Which column do you choose? (1-3)");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect(INPUT_ERR);
        let col = buf.trim().parse::<usize>()?;
        Ok((row, col))
    }

    fn is_coord_valid(coord: (usize, usize)) -> bool {
        coord.0 > 0 && coord.0 < 4 && coord.1 > 0 && coord.1 < 4
    }

    fn switch_current_player(&mut self) {
        if self.current_player == PlayerMark::Zero {
            self.current_player = PlayerMark::Cross
        } else {
            self.current_player = PlayerMark::Zero
        }
    }

    fn get_current_player_name(&self) -> String {
        match self.current_player {
            PlayerMark::Zero => "Zero".to_owned(),
            PlayerMark::Cross => "Cross".to_owned(),
        }
    }

    fn get_random_first_player() -> PlayerMark {
        if rand::thread_rng().gen_bool(0.5) {
            PlayerMark::Zero
        } else {
            PlayerMark::Cross
        }
    }
}

impl PartialEq<FieldState> for PlayerMark {
    fn eq(&self, other: &FieldState) -> bool {
        if self == &FieldState::Zero && other == &PlayerMark::Zero {
            return true;
        }

        if self == &FieldState::Cross && other == &PlayerMark::Cross {
            return true;
        }
        false
    }
}
