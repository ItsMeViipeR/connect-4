use puissance_4::{game_config::Config, game_master::GameMaster};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = Config::run()?;

    GameMaster::run(config)
}
