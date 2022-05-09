use rand::distributions::Uniform;

use crate::types::{PokemonType, POKEMON_COUNT};
use crate::battle::Fighter;

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

fn get_effectiveness(attacker: usize, defender: usize) -> i32
{
    POKEMON[attacker][defender]
}

#[derive(Clone)]
pub struct Pokemon
{
    health: i32,
    damage: i32,
    kind: PokemonType,
}

impl Pokemon
{
    pub fn new(kind: PokemonType) -> Self
    {
        Pokemon {
            health: 80,
            damage: 40,
            kind
        }
    }

    pub fn generate_randomly() -> impl Iterator<Item = Self>
    {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(0 .. POKEMON_COUNT);
        (0..).map(move |_| Self::new(PokemonType::random(&mut rng, &die)))
    }

    pub fn color(&self) -> nannou::image::Rgb<u8>
    {
        self.kind.into()
    }

    fn reset(&mut self, kind: PokemonType)
    {
        self.health = 80;
        self.damage = 40;
        self.kind = kind;
    }

    fn take_damage(&mut self, damage: i32) -> bool
    {
        self.health -= damage;

        self.health <= 0
    }
}

impl Fighter for Pokemon
{
    fn should_fight(&self, defender: &Self) -> bool
    {
        self.kind != defender.kind
    }

    fn get_effectiveness(&self, defender: &Self) -> i32
    {
        get_effectiveness(self.kind.into(), defender.kind.into())
    }

    fn fight(&self, defender: &mut Self) -> bool
    {
        let effectiveness = self.get_effectiveness(defender);
        let damage = self.damage * effectiveness / 100;

        let is_dead = defender.take_damage(damage);
        if is_dead
        {
            defender.reset(self.kind);
        }
        is_dead
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_effectiveness()
    {
        assert_eq!(Pokemon::new(PokemonType::Normal).get_effectiveness(&Pokemon::new(PokemonType::Normal)), 100);
        assert_eq!(Pokemon::new(PokemonType::Fire).get_effectiveness(&Pokemon::new(PokemonType::Steel)), 200);
        assert_eq!(Pokemon::new(PokemonType::Water).get_effectiveness(&Pokemon::new(PokemonType::Grass)), 50);
    }

    #[test]
    fn test_damage()
    {
        let mut p1 = Pokemon::new(PokemonType::Normal);
        let health = p1.health;
        let dead = p1.take_damage(40);

        assert_ne!(health, p1.health);
        assert!(!dead);

        let health = p1.health;
        let dead = p1.take_damage(40);

        assert_ne!(health, p1.health);
        assert!(dead);
    }

    #[test]
    fn test_reset()
    {
        let mut p1 = Pokemon::new(PokemonType::Normal);
        p1.reset(PokemonType::Fire);
        assert_eq!(p1.kind, PokemonType::Fire);

        let dead = p1.take_damage(80);
        assert!(dead);

        p1.reset(PokemonType::Dragon);
        assert_eq!(p1.kind, PokemonType::Dragon);

        let dead = p1.take_damage(40);
        assert!(!dead);
    }
}
