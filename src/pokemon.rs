use rand::prelude::ThreadRng;
use rand::distributions::Uniform;

use crate::{PokemonType, get_random_type};

pub const POKEMON_IMG_SIZE: usize = 800;
pub const POKEMON_COUNT: usize = 18;
pub const POKEMON: [[f32; POKEMON_COUNT]; POKEMON_COUNT] = [
	[ 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 0.0, 1.0, 1.0, 0.5, 1.0 ], // Normal
	[ 1.0, 0.5, 0.5, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 0.5, 1.0, 2.0, 1.0 ], // Fire
	[ 1.0, 2.0, 0.5, 1.0, 0.5, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 1.0, 1.0 ], // Water
	[ 1.0, 1.0, 2.0, 0.5, 0.5, 1.0, 1.0, 1.0, 0.0, 2.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0 ], // Electric
	[ 1.0, 0.5, 2.0, 1.0, 0.5, 1.0, 1.0, 0.5, 2.0, 0.5, 1.0, 0.5, 2.0, 1.0, 0.5, 1.0, 0.5, 1.0 ], // Grass
	[ 1.0, 0.5, 0.5, 1.0, 2.0, 0.5, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0 ], // Ice
	[ 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 0.5, 0.5, 0.5, 2.0, 0.0, 1.0, 2.0, 2.0, 0.5 ], // Fighting
	[ 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 0.5, 0.5, 1.0, 1.0, 1.0, 0.5, 0.5, 1.0, 1.0, 0.0, 2.0 ], // Poison
	[ 1.0, 2.0, 1.0, 2.0, 0.5, 1.0, 1.0, 2.0, 1.0, 0.0, 1.0, 0.5, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0 ], // Ground
	[ 1.0, 1.0, 1.0, 0.5, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 1.0, 1.0, 0.5, 1.0 ], // Flying
	[ 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 0.0, 0.5, 1.0 ], // Psychic
	[ 1.0, 0.5, 1.0, 1.0, 2.0, 1.0, 0.5, 0.5, 1.0, 0.5, 2.0, 1.0, 1.0, 0.5, 1.0, 2.0, 0.5, 0.5 ], // Bug
	[ 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 0.5, 2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0 ], // Rock
	[ 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 1.0 ], // Ghost
	[ 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 0.0 ], // Dragon
	[ 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 1.0, 0.5, 1.0, 0.5 ], // Dark
	[ 1.0, 0.5, 0.5, 0.5, 1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 0.5, 2.0 ], // Steel
	[ 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 2.0, 0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 0.5, 1.0 ]  // Fairy
];

pub fn get_effectiveness(attacker: usize, defender: usize) -> f32
{
    POKEMON[attacker][defender]
}

pub fn get_effectiveness_with_type(attacker: PokemonType, defender: PokemonType) -> f32
{
    get_effectiveness(attacker.into(), defender.into())
}

#[derive(Clone)]
pub struct Pokemon
{
    pub health: i32,
    pub damage: i32,
    pub kind: PokemonType,
}

impl Pokemon
{
    pub fn new(kind: PokemonType) -> Self
    {
        Pokemon {
            health: 80,
            damage: 40,
            kind,
        }
    }

    pub fn random(rng: &mut ThreadRng, die: &Uniform<usize>) -> Self
    {
        Pokemon {
            health: 80, 
            damage: 40,
            kind: get_random_type(rng, die)
        }
    }

    pub fn is_stronger(&self, other: &Pokemon) -> bool
    {
        get_effectiveness(self.kind.into(), other.kind.into()) > 1.0
    }

    pub fn take_damage(&mut self, damage: i32) -> bool
    {
        self.health -= damage;
        
        self.health <= 0
    }
}

#[cfg(test)]
mod tests {
    use crate::types::PokemonType;
    use crate::{Pokemon, get_effectiveness_with_type};

    #[test]
    fn test_get_effectiveness()
    {
        assert_eq!(get_effectiveness_with_type(PokemonType::Normal, PokemonType::Normal), 1.0);
        assert_eq!(get_effectiveness_with_type(PokemonType::Fire, PokemonType::Steel), 2.0);
        assert_eq!(get_effectiveness_with_type(PokemonType::Water, PokemonType::Grass), 0.5);
    }

    #[test]
    fn test_is_stronger()
    {
        {
            let p1 = Pokemon::new(PokemonType::Fire);
            let p2 = Pokemon::new(PokemonType::Steel);
            assert!(p1.is_stronger(&p2));
        }
        {
            let p1 = Pokemon::new(PokemonType::Normal);
            let p2 = Pokemon::new(PokemonType::Normal);
            assert!(!p1.is_stronger(&p2));
        }
        {
            let p1 = Pokemon::new(PokemonType::Water);
            let p2 = Pokemon::new(PokemonType::Grass);
            assert!(!p1.is_stronger(&p2));
        }
    }
}