use crate::battle::Fighter;
use crate::types::{Colored, GenerateRandomly, Generator};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use serde::Deserialize;
use strum::{EnumCount, FromRepr};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, EnumCount, FromRepr)]
#[repr(usize)]
pub enum PokemonType {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

impl From<PokemonType> for nannou::image::Rgb<u8> {
    fn from(kind: PokemonType) -> Self {
        match kind {
            PokemonType::Normal => [168, 168, 120],
            PokemonType::Fire => [240, 128, 48],
            PokemonType::Water => [104, 144, 240],
            PokemonType::Electric => [248, 208, 48],
            PokemonType::Grass => [120, 200, 80],
            PokemonType::Ice => [152, 216, 216],
            PokemonType::Fighting => [192, 48, 40],
            PokemonType::Poison => [160, 64, 160],
            PokemonType::Ground => [224, 192, 104],
            PokemonType::Flying => [168, 144, 240],
            PokemonType::Psychic => [248, 88, 136],
            PokemonType::Bug => [168, 184, 32],
            PokemonType::Rock => [184, 160, 56],
            PokemonType::Ghost => [112, 88, 152],
            PokemonType::Dragon => [112, 56, 248],
            PokemonType::Dark => [112, 88, 72],
            PokemonType::Steel => [184, 184, 208],
            PokemonType::Fairy => [240, 182, 188],
        }
        .into()
    }
}

#[rustfmt::skip]
const EFFICIENCY: [[i32; PokemonType::COUNT]; PokemonType::COUNT] = [
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

pub fn get_effectiveness(attacker: PokemonType, defender: PokemonType) -> i32 {
    EFFICIENCY[attacker as usize][defender as usize]
}

#[derive(Clone)]
pub struct Pokemon {
    health: i32,
    damage: i32,
    kind: PokemonType,
}

impl Pokemon {
    pub fn new(kind: PokemonType) -> Self {
        Pokemon {
            health: 80,
            damage: 40,
            kind,
        }
    }

    fn reset(&mut self, kind: PokemonType) {
        self.health = 80;
        self.damage = 40;
        self.kind = kind;
    }

    fn take_damage(&mut self, damage: i32) -> bool {
        self.health -= damage;

        self.health <= 0
    }
}

impl Fighter for Pokemon {
    fn should_fight(&self, defender: &Self) -> bool {
        self.kind != defender.kind
    }

    fn get_effectiveness(&self, defender: &Self) -> i32 {
        get_effectiveness(self.kind, defender.kind)
    }

    fn fight(&self, defender: &mut Self) -> bool {
        let effectiveness = self.get_effectiveness(defender);
        let damage = self.damage * effectiveness / 100;

        let is_dead = defender.take_damage(damage);
        if is_dead {
            defender.reset(self.kind);
        }
        is_dead
    }
}

pub struct PokemonGenerator {
    distribution: Uniform<usize>,
}

impl Default for PokemonGenerator {
    fn default() -> Self {
        Self {
            distribution: Uniform::new(0, PokemonType::COUNT),
        }
    }
}

impl GenerateRandomly<Pokemon> for PokemonGenerator {
    fn generate_randomly<R>(&self, rng: &mut R) -> Option<Pokemon>
    where
        R: Rng,
    {
        let t = self.distribution.sample(rng);
        let kind = PokemonType::from_repr(t)?;
        Some(Pokemon::new(kind))
    }
}

impl Generator for Pokemon {
    type Generator = PokemonGenerator;

    fn generator() -> Self::Generator {
        PokemonGenerator::default()
    }
}

impl Colored for Pokemon {
    fn color(&self) -> nannou::image::Rgb<u8> {
        self.kind.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_type_from_usize() {
        assert_eq!(PokemonType::from_repr(0).unwrap(), PokemonType::Normal);
        assert_eq!(PokemonType::from_repr(17).unwrap(), PokemonType::Fairy);
    }

    #[test]
    fn test_get_effectiveness() {
        assert_eq!(
            Pokemon::new(PokemonType::Normal).get_effectiveness(&Pokemon::new(PokemonType::Normal)),
            100
        );
        assert_eq!(
            Pokemon::new(PokemonType::Fire).get_effectiveness(&Pokemon::new(PokemonType::Steel)),
            200
        );
        assert_eq!(
            Pokemon::new(PokemonType::Water).get_effectiveness(&Pokemon::new(PokemonType::Grass)),
            50
        );
    }

    #[test]
    fn test_get_color() {
        assert_eq!(
            Pokemon::new(PokemonType::Normal).color(),
            nannou::image::Rgb([168, 168, 120])
        );
        assert_eq!(
            Pokemon::new(PokemonType::Fairy).color(),
            nannou::image::Rgb([240, 182, 188])
        );
    }

    #[test]
    fn test_damage() {
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
    fn test_reset() {
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
