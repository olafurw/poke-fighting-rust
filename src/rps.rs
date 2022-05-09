use nannou::image::Rgb;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;
use strum::{FromRepr,EnumCount};


use crate::battle::Fighter;

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumCount, FromRepr)]
#[repr(usize)]
pub enum RPSType
{
    Rock,
    Paper,
    Scissor,
}

pub const RPS_COUNT: usize = RPSType::COUNT;

impl RPSType
{
    pub fn random(rng: &mut ThreadRng, die: &Uniform<usize>) -> Self
    {
        let value = die.sample(rng);
        RPSType::from_repr(value).unwrap()
    }
}

impl From<RPSType> for usize
{
    fn from(kind: RPSType) -> Self
    {
        kind as Self
    }
}

impl From<RPSType> for Rgb<u8>
{
    fn from(kind: RPSType) -> Self
    {
        match kind {
            RPSType::Rock => [168, 168, 120],
            RPSType::Paper => [240, 128, 48],
            RPSType::Scissor => [104, 144, 240],
        }
        .into()
    }
}

fn get_effectiveness(attacker: usize, defender: usize) -> i32
{
    let attacker_type = RPSType::from_repr(attacker).unwrap();
    let defender_type = RPSType::from_repr(defender).unwrap();

    // should be a table.
    if attacker_type == RPSType::Rock
    {
        if defender_type == RPSType::Scissor
        {
            1
        }
        else
        {
            0
        }
    }
    else if attacker_type == RPSType::Paper
    {
        if defender_type == RPSType::Rock
        {
            1
        }
        else
        {
            0
        }
    }
    else
    {
        if defender_type == RPSType::Rock
        {
            1
        }
        else
        {
            0
        }
    }
}

#[derive(Clone)]
pub struct RPS
{
    health: i32,
    damage: i32,
    kind: RPSType,
}

impl RPS
{
    pub fn new(kind: RPSType) -> Self
    {
        RPS {
            health: 1,
            damage: 1,
            kind,
        }
    }

    pub fn generate_randomly() -> impl Iterator<Item = Self>
    {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(0..RPS_COUNT);
        (0..).map(move |_| Self::new(RPSType::random(&mut rng, &die)))
    }

    pub fn color(&self) -> nannou::image::Rgb<u8>
    {
        self.kind.into()
    }

    fn reset(&mut self, kind: RPSType)
    {
        self.health = 1;
        self.damage = 1;
        self.kind = kind;
    }

    fn take_damage(&mut self, damage: i32) -> bool
    {
        self.health -= damage;

        self.health <= 0
    }
}

impl Fighter for RPS
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
        assert_eq!(
            RPS::new(RPSType::Rock).get_effectiveness(&RPS::new(RPSType::Scissor)),
            1
        );
    }

    #[test]
    fn test_damage()
    {
        let mut p1 = RPS::new(RPSType::Rock);
        let health = p1.health;
        let dead = p1.take_damage(0);

        assert_eq!(health, p1.health);
        assert!(!dead);

        let health = p1.health;
        let dead = p1.take_damage(1);

        assert_ne!(health, p1.health);
        assert!(dead);
    }

    #[test]
    fn test_reset()
    {
        let mut p1 = RPS::new(RPSType::Rock);
        p1.reset(RPSType::Paper);
        assert_eq!(p1.kind, RPSType::Paper);

        let dead = p1.take_damage(1);
        assert!(dead);

        p1.reset(RPSType::Scissor);
        assert_eq!(p1.kind, RPSType::Scissor);

        let dead = p1.take_damage(0);
        assert!(!dead);
    }
}
