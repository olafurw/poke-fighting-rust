#![allow(dead_code)]

use rand::distributions::Uniform;
use std::fs::File;
use std::io::Write;

mod types;
use crate::types::*;

mod pokemon;
use crate::pokemon::*;

mod battle;
use crate::battle::*;

fn main() -> std::io::Result<()>
{
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0 .. POKEMON_COUNT);

    let mut battle = Battle::new(&mut rng, &die);
    for i in 0..200
    {
        let _ = battle.action();
        let mut file = File::create(format!("battle/file_{:04}.txt", i))?;
        write!(file, "{}", battle)?;
    }

    Ok(())
}
