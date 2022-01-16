use std::error::Error;
use std::io::{self, Write};

#[derive(Clone, PartialEq, Copy)]
pub enum Mode {
    Solo,
    Multi,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PlayerNb {
    P1,
    P2,
}

#[derive(Clone, PartialEq, Copy)]
pub enum PlayerKind {
    User,
    Computer,
}

#[derive(Clone, PartialEq, Copy)]
pub struct Player {
    pub nb: PlayerNb,
    pub kind: PlayerKind,
}

#[derive(Clone, PartialEq, Copy)]
pub struct Config {
    pub mode: Mode,
    pub p1: Player,
    pub p2: Player,
}

impl Config {
    fn mode() -> Result<Mode, Box<dyn Error>> {
        println!(
            "Please, choose your game mode\n\
            s: solo player\n\
            m: multiplayer\n"
        );

        io::stdout().flush()?;

        let mut input: String = String::new();

        io::stdin().read_line(&mut input)?;

        match input.to_lowercase().trim() {
            "s" => {
                println!("The solo mode isn't available.");

                return Self::mode();
            }
            "m" => Ok(Mode::Multi),
            _ => {
                println!(
                    "\"{}\" is an invalid input. Please try again.\n",
                    input.trim()
                );

                return Self::mode();
            }
        }
    }

    fn player_solo_mode() -> Result<(Player, Player), Box<dyn Error>> {
        let mut p1 = Player {
            nb: PlayerNb::P1,
            kind: PlayerKind::User,
        };
        let mut p2 = Player {
            nb: PlayerNb::P2,
            kind: PlayerKind::User,
        };

        println!(
            "What player do you want to be?\n\
            1: player 1\n\
            2: player 2\n"
        );

        io::stdout().flush()?;

        let mut input: String = String::new();

        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => {
                p2.kind = PlayerKind::Computer;
                Ok((p1, p2))
            }
            "2" => {
                p1.kind = PlayerKind::Computer;

                Ok((p1, p2))
            }
            _ => {
                println!(
                    "\"{}\" is an invalid input. Please try again.\n",
                    input.trim()
                );

                return Self::player_solo_mode();
            }
        }
    }

    fn player_multi_mode() -> (Player, Player) {
        (
            Player {
                nb: PlayerNb::P1,
                kind: PlayerKind::User,
            },
            Player {
                nb: PlayerNb::P2,
                kind: PlayerKind::User,
            },
        )
    }

    pub fn run() -> Result<Self, Box<dyn Error>> {
        println!("Welcome to Power4!\n");

        let mode: Mode = Self::mode()?;
        let players: (Player, Player);

        match mode {
            Mode::Solo => players = Self::player_solo_mode()?,
            Mode::Multi => players = Self::player_multi_mode(),
        };

        Ok(Config {
            mode,
            p1: players.0,
            p2: players.1,
        })
    }
}
