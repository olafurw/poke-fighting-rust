#![allow(dead_code)]

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
    let mut battle = Battle::new();
    for i in 0..10000
    {
        let _ = battle.action();
        let mut file = File::create(format!("battle/file_{:06}.txt", i))?;
        write!(file, "{}", battle)?;
    }

    Ok(())
}
