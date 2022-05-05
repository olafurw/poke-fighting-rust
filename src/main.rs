#![allow(dead_code)]

use rand::distributions::Uniform;

mod types;
use crate::types::*;

mod pokemon;
use crate::pokemon::*;

mod battle;
use crate::battle::*;

fn main()
{
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0 .. POKEMON_COUNT);

    let mut battle = Battle::new(&mut rng, &die);
    let death_count = battle.action();

    println!("Deaths: {}", death_count);
}
