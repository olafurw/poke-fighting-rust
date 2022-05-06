use rand::prelude::ThreadRng;
use rand::distributions::Uniform;

use crate::PokemonType;

pub const POKEMON_COUNT: usize = 18;
pub const POKEMON: [[i32; POKEMON_COUNT]; POKEMON_COUNT] = [
	[ 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,  50,   0, 100, 100,  50, 100 ], // Normal
	[ 100,  50,  50, 100, 200, 200, 100, 100, 100, 100, 100, 200,  50, 100,  50, 100, 200, 100 ], // Fire
	[ 100, 200,  50, 100,  50, 100, 100, 100, 200, 100, 100, 100, 200, 100,  50, 100, 100, 100 ], // Water
	[ 100, 100, 200,  50,  50, 100, 100, 100,   0, 200, 100, 100, 100, 100,  50, 100, 100, 100 ], // Electric
	[ 100,  50, 200, 100,  50, 100, 100,  50, 200,  50, 100,  50, 200, 100,  50, 100,  50, 100 ], // Grass
	[ 100,  50,  50, 100, 200,  50, 100, 100, 200, 200, 100, 100, 100, 100, 200, 100,  50, 100 ], // Ice
	[ 200, 100, 100, 100, 100, 200, 100,  50, 100,  50,  50,  50, 200,   0, 100, 200, 200,  50 ], // Fighting
	[ 100, 100, 100, 100, 200, 100, 100,  50,  50, 100, 100, 100,  50,  50, 100, 100,   0, 200 ], // Poison
	[ 100, 200, 100, 200,  50, 100, 100, 200, 100,   0, 100,  50, 200, 100, 100, 100, 200, 100 ], // Ground
	[ 100, 100, 100,  50, 200, 100, 200, 100, 100, 100, 100, 200,  50, 100, 100, 100,  50, 100 ], // Flying
	[ 100, 100, 100, 100, 100, 100, 200, 200, 100, 100,  50, 100, 100, 100, 100,   0,  50, 100 ], // Psychic
	[ 100,  50, 100, 100, 200, 100,  50,  50, 100,  50, 200, 100, 100,  50, 100, 200,  50,  50 ], // Bug
	[ 100, 200, 100, 100, 100, 200,  50, 100,  50, 200, 100, 200, 100, 100, 100, 100,  50, 100 ], // Rock
	[   0, 100, 100, 100, 100, 100, 100, 100, 100, 100, 200, 100, 100, 200, 100,  50, 100, 100 ], // Ghost
	[ 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 200, 100,  50,   0 ], // Dragon
	[ 100, 100, 100, 100, 100, 100,  50, 100, 100, 100, 200, 100, 100, 200, 100,  50, 100,  50 ], // Dark
	[ 100,  50,  50,  50, 100, 200, 100, 100, 100, 100, 100, 100, 200, 100, 100, 100,  50, 200 ], // Steel
	[ 100,  50, 100, 100, 100, 100, 200,  50, 100, 100, 100, 100, 100, 100, 200, 200,  50, 100 ]  // Fairy
];

pub fn get_effectiveness(attacker: usize, defender: usize) -> i32
{
    POKEMON[attacker][defender]
}

pub fn get_effectiveness_with_type(attacker: PokemonType, defender: PokemonType) -> i32
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
    pub fn random(rng: &mut ThreadRng, die: &Uniform<usize>) -> Self
    {
        Pokemon {
            health: 80,
            damage: 40,
            kind: PokemonType::random(rng, die)
        }
    }

    pub fn reset(&mut self, kind: PokemonType)
    {
        self.health = 80;
        self.damage = 40;
        self.kind = kind;
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
    use crate::get_effectiveness_with_type;

    #[test]
    fn test_get_effectiveness()
    {
        assert_eq!(get_effectiveness_with_type(PokemonType::Normal, PokemonType::Normal), 100);
        assert_eq!(get_effectiveness_with_type(PokemonType::Fire, PokemonType::Steel), 200);
        assert_eq!(get_effectiveness_with_type(PokemonType::Water, PokemonType::Grass),  50);
    }
}
