use crate::game_config::{Config, Player, PlayerKind, PlayerNb};
use colored::Colorize;
use std::{
    convert::TryInto,
    error::Error,
    fmt,
    io::{self, Write},
};

const COL: usize = 7;
const ROW: usize = 6;
const NB_TURNS: usize = COL * ROW;

#[derive(Clone, PartialEq, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

pub struct GameMaster {
    grid: Vec<Vec<Option<PlayerNb>>>,
    p1: Player,
    p2: Player,
    turn: PlayerNb,
    nb_turn: usize,
}

impl GameMaster {
    pub fn new(config: Config) -> Self {
        Self {
            grid: vec![vec![None; COL]; ROW],
            p1: config.p1,
            p2: config.p2,
            turn: PlayerNb::P1,
            nb_turn: 0,
        }
    }

    fn check_success(&self, pos: Position) -> bool {
        let Position { x, y } = pos;
        let directions: [(i32, i32); 4] = [(1, 0), (0, 1), (1, 1), (1, -1)];

        directions.iter().any(|&(dx, dy)| {
            (0..4).any(|start| {
                (0..4)
                    .map(|i| {
                        let col: usize = (x as i32 + (i - start) * dx).try_into().ok()?;
                        let row: usize = (y as i32 + (i - start) * dy).try_into().ok()?;
                        *self.grid.get(row)?.get(col)?
                    })
                    .all(|v| v == Some(self.turn))
            })
        })
    }

    fn check_column(&self, input: String) -> Result<Position, ColError> {
        let res = input.parse();
        let col: usize;
        match res {
            Ok(nb) => col = nb,
            _ => return Err(ColError::Invalid(input)),
        }
        if col < 1 || col > 8 {
            return Err(ColError::WrongColNb(col));
        }
        let mut row = ROW - 1;
        loop {
            if self.grid[row][col - 1].is_none() {
                return Ok(Position { x: col - 1, y: row });
            }
            if row == 0 {
                break;
            }
            row -= 1;
        }
        Err(ColError::FullCol(col))
    }

    fn check_full(&self) -> bool {
        self.nb_turn == NB_TURNS
    }

    fn fill_grid(&mut self, player: PlayerNb, pos: Position) {
        self.grid[pos.y][pos.x] = Some(player);
    }

    fn display_grid(&self) {
        println!("\n  1   2   3   4   5   6   7  ");
        println!("|---+---+---+---+---+---+---|");
        for row in self.grid.iter() {
            print!("|");
            for val in row.iter() {
                match val {
                    None => print!("   |"),
                    Some(p) => print!(
                        " {} |",
                        if *p == PlayerNb::P1 {
                            format!("O").bold().red()
                        } else {
                            format!("X").bold().blue()
                        }
                    ),
                }
            }
            println!("\n|---+---+---+---+---+---+---|");
        }
        println!(" ");
    }

    fn process_computer_turn(&self) -> Position {
        unimplemented!();
    }

    fn process_user_turn(&self) -> Result<Position, Box<dyn Error>> {
        println!(
            "{}, it's your turn.\nPlease choose a column.\n",
            if self.turn == PlayerNb::P1 {
                format!("{:?}", self.turn).bold().red()
            } else {
                format!("{:?}", self.turn).bold().blue()
            }
        );
        io::stdout().flush()?;
        let pos: Position;
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            match self.check_column(input[..].trim().to_string()) {
                Err(e) => println!("{}\nPlease try again.\n", e),
                Ok(p) => {
                    pos = p;
                    break;
                }
            }
        }
        Ok(pos)
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let mut game_master = GameMaster::new(config);
        println!("\nHere the game begins !\n");
        loop {
            game_master.display_grid();
            let pos: Position;
            if (game_master.turn == PlayerNb::P1 && game_master.p1.kind == PlayerKind::Computer)
                || (game_master.turn == PlayerNb::P2 && game_master.p2.kind == PlayerKind::Computer)
            {
                pos = game_master.process_computer_turn();
            } else {
                pos = game_master.process_user_turn()?;
            }
            game_master.fill_grid(game_master.turn, pos);
            game_master.nb_turn += 1;
            if game_master.check_success(pos) {
                game_master.display_grid();
                println!("Congrats {:?}, you won !\n", game_master.turn);
                return Ok(());
            }
            if game_master.check_full() {
                game_master.display_grid();
                println!("It's a draw !\n");
                return Ok(());
            }
            match game_master.turn {
                PlayerNb::P1 => game_master.turn = PlayerNb::P2,
                PlayerNb::P2 => game_master.turn = PlayerNb::P1,
            }
        }
    }
}

pub enum ColError {
    Invalid(String),
    WrongColNb(usize),
    FullCol(usize),
}

impl fmt::Display for ColError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ColError::Invalid(s) => write!(f, "\"{}\" is an invalid proposition.", (s)),
            ColError::WrongColNb(nb) => write!(
                f,
                "{} is not a correct column number.\n\
                    You should choose a number between 1 and 8 (included).",
                nb
            ),
            ColError::FullCol(nb) => {
                write!(f, "Column {} is full. You have to choose another one.", nb)
            }
        }
    }
}

impl fmt::Debug for ColError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for ColError {}

#[cfg(test)]
mod tests {
    use crate::game_config::{
        Player, PlayerKind,
        PlayerNb::{self, P1, P2},
    };
    use crate::game_master::{GameMaster, Position, COL, ROW};

    const A: Option<PlayerNb> = Some(P1);
    const B: Option<PlayerNb> = Some(P2);
    const O: Option<PlayerNb> = None;

    fn assert_success_3_2(grid: Vec<Vec<Option<PlayerNb>>>) {
        assert_eq!(true, make_grid(grid).check_success(Position { x: 3, y: 2 }));
    }

    fn assert_no_success_3_2(grid: Vec<Vec<Option<PlayerNb>>>) {
        assert_eq!(
            false,
            make_grid(grid).check_success(Position { x: 3, y: 2 })
        );
    }

    fn make_grid(grid: Vec<Vec<Option<PlayerNb>>>) -> GameMaster {
        assert_eq!(grid.len(), ROW);
        assert_eq!(grid[0].len(), COL);
        GameMaster {
            grid,
            p1: Player {
                nb: P1,
                kind: PlayerKind::User,
            },
            p2: Player {
                nb: P2,
                kind: PlayerKind::User,
            },
            turn: P1,
            nb_turn: 0,
        }
    }

    #[test]
    fn test_check_empty_grid() {
        assert_no_success_3_2(vec![
            vec![O, O, O, O, O, O, O],
            vec![O, O, O, O, O, O, O],
            vec![O, O, O, O, O, O, O],
            vec![O, O, O, O, O, O, O],
            vec![O, O, O, O, O, O, O],
            vec![O, O, O, O, O, O, O],
        ]);
    }

    #[test]
    fn test_check_success_vertical() {
        assert_success_3_2(vec![
            vec![O, O, O, O, O, O, O],
            vec![O, O, O, O, O, O, O],
            vec![O, O, O, A, O, O, O],
            vec![O, O, O, A, O, O, O],
            vec![O, O, O, A, O, O, O],
            vec![O, O, O, A, O, O, O],
        ]);
    }

    #[test]
    fn test_check_success_vertical_top() {
        assert_success_3_2(vec![
            vec![O, O, O, A, O, O, O],
            vec![O, O, O, A, O, O, O],
            vec![O, O, O, A, O, O, O],
            vec![O, O, O, A, O, O, O],
            vec![O, O, O, B, O, O, O],
            vec![O, O, O, B, O, O, O],
        ]);
    }

    #[test]
    fn test_check_no_success_vertical_non_continuous() {
        assert_no_success_3_2(vec![
            vec![O, O, O, O, O, O, O],
            vec![O, O, O, A, O, O, O],
            vec![O, O, O, B, O, O, O],
            vec![O, O, O, A, O, O, O],
            vec![O, O, O, A, O, O, O],
            vec![O, O, O, A, O, O, O],
        ]);
    }

    #[test]
    fn test_check_success_horizontal() {
        assert_success_3_2(vec![
            vec![O, O, O, A, O, O, O],
            vec![O, O, O, B, O, O, O],
            vec![O, A, A, A, A, O, O],
            vec![O, B, B, A, A, O, O],
            vec![O, A, B, B, B, O, O],
            vec![O, B, B, B, A, O, O],
        ]);
    }

    #[test]
    fn test_check_no_success_horizontal_missing() {
        assert_no_success_3_2(vec![
            vec![O, O, O, A, O, O, O],
            vec![O, O, O, B, O, O, O],
            vec![O, B, A, A, A, O, O],
            vec![O, B, B, A, A, O, O],
            vec![O, A, B, B, B, O, O],
            vec![O, B, B, B, A, O, O],
        ]);
    }

    #[test]
    fn test_check_success_diagonal1() {
        assert_success_3_2(vec![
            vec![O, O, O, A, O, O, O],
            vec![O, O, O, B, O, O, O],
            vec![O, B, A, A, A, O, O],
            vec![O, B, B, A, A, O, O],
            vec![O, A, B, B, B, A, O],
            vec![O, B, B, B, A, A, A],
        ]);
    }

    #[test]
    fn test_check_success_diagonal2() {
        assert_success_3_2(vec![
            vec![O, O, O, A, O, O, O],
            vec![O, O, O, B, A, O, O],
            vec![O, B, O, A, A, O, O],
            vec![O, B, A, A, A, O, O],
            vec![O, A, B, B, B, O, O],
            vec![O, B, B, B, A, O, O],
        ]);
    }
}
