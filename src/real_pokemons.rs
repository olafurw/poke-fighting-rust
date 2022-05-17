use crate::pokemon::{get_effectiveness, PokemonType};
use crate::types::GenerateRandomly;
use crate::{Colored, Fighter, Generator};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use serde::Deserialize;
use std::cmp::max;

#[derive(Debug, Clone)]
pub struct RealPokemon {
    name: String,
    type1: PokemonType,
    type2: Option<PokemonType>,
    hp: i32,
    attack: i32,
    defense: i32,
    sp_attack: i32,
    sp_defense: i32,
    speed: i32,
}

impl Fighter for RealPokemon {
    fn should_fight(&self, defender: &Self) -> bool {
        self.name != defender.name
    }

    fn get_effectiveness(&self, defender: &Self) -> i32 {
        let type2_effectiveness = if let Some(type2) = defender.type2 {
            get_effectiveness(self.type1, type2)
        } else {
            100
        };

        get_effectiveness(self.type1, defender.type1) * type2_effectiveness
    }

    fn fight(&self, defender: &mut Self) -> bool {
        // Starting with the fastest pokemon, each pokemon will deal damage
        // based on a simplified version of https://bulbapedia.bulbagarden.net/wiki/Damage
        // up until one of them faint.
        //
        // If the defender faint, it converts to the attacker, otherwise, nothing
        // happens.

        let mut attacker_hp = self.hp;
        let mut defender_hp = self.hp;

        // Give attacker priority on speed-ties
        if defender.speed > self.speed {
            fight(defender, self, &mut attacker_hp);
        }

        loop {
            if attacker_hp < 0 || defender_hp < 0 {
                break;
            }

            fight(self, defender, &mut defender_hp);

            if attacker_hp < 0 || defender_hp < 0 {
                break;
            }

            fight(defender, self, &mut attacker_hp);
        }

        if defender_hp < 0 {
            *defender = self.clone();
            true
        } else {
            false
        }
    }
}

fn fight(attacker: &RealPokemon, defender: &RealPokemon, defender_hp: &mut i32) {
    // We use https://bulbapedia.bulbagarden.net/wiki/Damage to compute damage,
    // assuming a level 5, Power 40 move that uses the best stat the attacker have,
    // and the first type of the attacker, without STAB

    let ad = max(
        1000 * attacker.attack / defender.defense,
        1000 * attacker.sp_attack / defender.sp_defense,
    );

    let eff = attacker.get_effectiveness(defender);

    // We compensate the x1000 attack / defense ratio
    // by dividing by 50 * 1000
    // Effectiveness is 100x100 (two types), so we need to divide too
    let damage = 4 * 40 * ad / 50000 * eff / 10000 + 2;
    *defender_hp -= damage;
}

pub struct RealPokemonGenerator {
    pokemons: Vec<RealPokemon>,
    distribution: Uniform<usize>,
}

impl Default for RealPokemonGenerator {
    fn default() -> Self {
        let pokemons = load_pokemons()
            .into_iter()
            .filter_map(map_pokemon)
            .collect::<Vec<_>>();

        let count = pokemons.len();
        Self {
            pokemons,
            distribution: Uniform::new(0, count),
        }
    }
}

impl GenerateRandomly<RealPokemon> for RealPokemonGenerator {
    fn generate_randomly<R>(&self, rng: &mut R) -> Option<RealPokemon>
    where
        R: Rng,
    {
        let i = self.distribution.sample(rng);
        self.pokemons.get(i).cloned()
    }
}

impl Generator for RealPokemon {
    type Generator = RealPokemonGenerator;

    fn generator() -> Self::Generator {
        RealPokemonGenerator::default()
    }
}

impl Colored for RealPokemon {
    fn color(&self) -> nannou::image::Rgb<u8> {
        self.type1.into()
    }
}

#[derive(Deserialize)]
struct PokemonJson {
    name: PokemonJsonNames,
    #[serde(rename = "type")]
    types: Vec<PokemonType>,
    base: Option<PokemonJsonBase>,
}

#[derive(Deserialize)]
struct PokemonJsonNames {
    english: String,
}

#[derive(Deserialize)]
struct PokemonJsonBase {
    #[serde(rename = "HP")]
    hp: i32,
    #[serde(rename = "Attack")]
    attack: i32,
    #[serde(rename = "Defense")]
    defense: i32,
    #[serde(rename = "Sp. Attack")]
    sp_attack: i32,
    #[serde(rename = "Sp. Defense")]
    sp_defense: i32,
    #[serde(rename = "Speed")]
    speed: i32,
}

fn load_pokemons() -> Vec<PokemonJson> {
    let client = reqwest::blocking::Client::default();
    let response = client
        .get("https://raw.githubusercontent.com/Purukitto/pokemon-data.json/master/pokedex.json")
        .send()
        .unwrap();

    response.json().unwrap()
}

fn map_pokemon(pokemon: PokemonJson) -> Option<RealPokemon> {
    let base = pokemon.base?;
    let type1 = *pokemon.types.get(0)?;
    Some(RealPokemon {
        name: pokemon.name.english,
        type1,
        type2: pokemon.types.get(1).copied(),
        hp: base.hp,
        attack: base.attack,
        defense: base.defense,
        sp_attack: base.sp_attack,
        sp_defense: base.sp_defense,
        speed: base.speed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deserialize() {
        load_pokemons();
    }
}
