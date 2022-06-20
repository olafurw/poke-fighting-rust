use crate::battle::Fighter;
use crate::types::{Colored, GenerateRandomly};
use lazy_static::lazy_static;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use strum::{EnumCount, FromRepr};

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumCount, FromRepr)]
#[repr(usize)]
pub enum RPSType {
    Rock,
    Paper,
    Scissor,
}

impl From<usize> for RPSType {
    fn from(repr: usize) -> Self {
        Self::from_repr(repr).unwrap()
    }
}

impl From<RPSType> for usize {
    fn from(kind: RPSType) -> Self {
        kind as Self
    }
}

impl From<RPSType> for nannou::image::Rgb<u8> {
    fn from(kind: RPSType) -> Self {
        match kind {
            RPSType::Rock => [128, 0, 0],
            RPSType::Paper => [0, 0, 128],
            RPSType::Scissor => [0, 128, 0],
        }
        .into()
    }
}

#[rustfmt::skip]
const EFFICIENCY: [[i32; RPSType::COUNT]; RPSType::COUNT] = [
    [   0,   0, 100 ], // Rock
    [ 100,   0,   0 ], // Paper
    [   0, 100,   0 ], // Scissor
];

fn get_effectiveness(attacker: RPSType, defender: RPSType) -> i32 {
    EFFICIENCY[attacker as usize][defender as usize]
}

#[derive(Clone)]
pub struct RPS {
    health: i32,
    damage: i32,
    kind: RPSType,
}

impl RPS {
    pub fn new(kind: RPSType) -> Self {
        RPS {
            health: 100,
            damage: 100,
            kind,
        }
    }

    fn reset(&mut self, kind: RPSType) {
        self.health = 100;
        self.damage = 100;
        self.kind = kind;
    }

    fn take_damage(&mut self, damage: i32) -> bool {
        self.health -= damage;

        self.health <= 0
    }
}

impl Fighter for RPS {
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

lazy_static! {
    static ref DISTRIBUTION: Uniform<usize> = Uniform::new(0, RPSType::COUNT);
}

impl GenerateRandomly for RPS {
    fn generate_randomly<R>(rng: &mut R) -> Self
    where
        R: Rng,
    {
        let t = DISTRIBUTION.sample(rng);
        Self::new(t.into())
    }
}

impl Colored for RPS {
    fn color(&self) -> nannou::image::Rgb<u8> {
        self.kind.into()
    }
}

impl core::fmt::Display for RPS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_effectiveness() {
        assert_eq!(
            RPS::new(RPSType::Rock).get_effectiveness(&RPS::new(RPSType::Scissor)),
            100
        );
        assert_eq!(
            RPS::new(RPSType::Paper).get_effectiveness(&RPS::new(RPSType::Rock)),
            100
        );
        assert_eq!(
            RPS::new(RPSType::Scissor).get_effectiveness(&RPS::new(RPSType::Paper)),
            100
        );

        assert_eq!(
            RPS::new(RPSType::Scissor).get_effectiveness(&RPS::new(RPSType::Rock)),
            0
        );
        assert_eq!(
            RPS::new(RPSType::Rock).get_effectiveness(&RPS::new(RPSType::Paper)),
            0
        );
        assert_eq!(
            RPS::new(RPSType::Paper).get_effectiveness(&RPS::new(RPSType::Scissor)),
            0
        );

        assert_eq!(
            RPS::new(RPSType::Scissor).get_effectiveness(&RPS::new(RPSType::Scissor)),
            0
        );
        assert_eq!(
            RPS::new(RPSType::Rock).get_effectiveness(&RPS::new(RPSType::Rock)),
            0
        );
        assert_eq!(
            RPS::new(RPSType::Paper).get_effectiveness(&RPS::new(RPSType::Paper)),
            0
        );
    }

    #[test]
    fn test_get_color() {
        assert_eq!(
            RPS::new(RPSType::Rock).color(),
            nannou::image::Rgb([128, 0, 0])
        );
        assert_eq!(
            RPS::new(RPSType::Paper).color(),
            nannou::image::Rgb([0, 0, 128])
        );
        assert_eq!(
            RPS::new(RPSType::Scissor).color(),
            nannou::image::Rgb([0, 128, 0])
        );
    }

    #[test]
    fn test_damage() {
        let mut p1 = RPS::new(RPSType::Rock);
        let health = p1.health;
        let dead = p1.take_damage(0);

        assert_eq!(health, p1.health);
        assert!(!dead);

        let health = p1.health;
        let dead = p1.take_damage(100);

        assert_ne!(health, p1.health);
        assert!(dead);
    }

    #[test]
    fn test_reset() {
        let mut p1 = RPS::new(RPSType::Rock);
        p1.reset(RPSType::Paper);
        assert_eq!(p1.kind, RPSType::Paper);

        let dead = p1.take_damage(100);
        assert!(dead);

        p1.reset(RPSType::Scissor);
        assert_eq!(p1.kind, RPSType::Scissor);

        let dead = p1.take_damage(0);
        assert!(!dead);
    }
}
