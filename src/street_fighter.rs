// Data for Super Street Fighter 4 Arcade Edition v2012
// https://www.eventhubs.com/guides/2008/oct/17/street-fighter-4-tiers-character-rankings/
// Why that one? Only page I found with a table I could copy paste, all newer rankings are images
// And OCR does not handle them well.

use crate::battle::Fighter;
use crate::types::{Colored, RandomlyGeneratable};
use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;
use rand::Rng;
use strum::{EnumCount, FromRepr};

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumCount, FromRepr)]
#[repr(usize)]
pub enum StreetFighterType {
    Seth,
    CViper,
    Cammy,
    Akuma,
    FLong,
    Rufus,
    Sagat,
    Balrog,
    Adon,
    Ibuki,
    Abel,
    Blanka,
    Makoto,
    Bison,
    Ryu,
    Ken,
    Yun,
    Zangief,
    Dhalsim,
    Guile,
    Sakura,
    ChunLi,
    DeeJay,
    Juri,
    Rose,
    Gouken,
    Guy,
    Cody,
    Fuerte,
    Yang,
    EHonda,
    Gen,
    Vega,
    Dudley,
    Oni,
    EvilRyu,
    Hakan,
    THawk,
    Dan,
}

impl StreetFighterType {
    pub fn random(rng: &mut ThreadRng, die: &Uniform<usize>) -> Self {
        let value = die.sample(rng);
        StreetFighterType::from_repr(value).unwrap()
    }
}

impl From<usize> for StreetFighterType {
    fn from(repr: usize) -> Self {
        Self::from_repr(repr).unwrap()
    }
}

impl From<StreetFighterType> for usize {
    fn from(kind: StreetFighterType) -> Self {
        kind as Self
    }
}

impl From<StreetFighterType> for nannou::image::Rgb<u8> {
    fn from(kind: StreetFighterType) -> Self {
        match kind {
            StreetFighterType::Seth => [100, 122, 4],
            StreetFighterType::CViper => [105, 78, 203], // blue
            StreetFighterType::Cammy => [107, 255, 138],
            StreetFighterType::Akuma => [136, 41, 110], // purple
            StreetFighterType::FLong => [145, 143, 47],
            StreetFighterType::Rufus => [15, 0, 158],
            StreetFighterType::Sagat => [158, 102, 221], // x
            StreetFighterType::Balrog => [170, 122, 61], // x
            StreetFighterType::Adon => [172, 195, 17],
            StreetFighterType::Ibuki => [172, 244, 210],
            StreetFighterType::Abel => [180, 0, 170],
            StreetFighterType::Blanka => [180, 234, 210],
            StreetFighterType::Makoto => [189, 94, 2],
            StreetFighterType::Bison => [196, 103, 77],
            StreetFighterType::Ryu => [196, 238, 203],
            StreetFighterType::Ken => [198, 250, 237],
            StreetFighterType::Yun => [20, 184, 104],
            StreetFighterType::Zangief => [236, 120, 183], // pink
            StreetFighterType::Dhalsim => [236, 231, 122],
            StreetFighterType::Guile => [240, 211, 242],
            StreetFighterType::Sakura => [242, 255, 43],
            StreetFighterType::ChunLi => [244, 214, 202],
            StreetFighterType::DeeJay => [254, 177, 238],
            StreetFighterType::Juri => [255, 139, 106],
            StreetFighterType::Rose => [27, 228, 98],
            StreetFighterType::Gouken => [56, 205, 99],
            StreetFighterType::Guy => [56, 43, 146],
            StreetFighterType::Cody => [75, 245, 255],
            StreetFighterType::Fuerte => [84, 123, 12],
            StreetFighterType::Yang => [96, 186, 0],
            StreetFighterType::EHonda => [241, 204, 245],
            StreetFighterType::Gen => [176, 43, 196],
            StreetFighterType::Vega => [175, 74, 207],
            StreetFighterType::Dudley => [116, 252, 200],
            StreetFighterType::Oni => [187, 241, 212],
            StreetFighterType::EvilRyu => [137, 215, 168],
            StreetFighterType::Hakan => [97, 148, 5],
            StreetFighterType::THawk => [250, 254, 145],
            StreetFighterType::Dan => [136, 51, 0],
        }
        .into()
    }
}

#[rustfmt::skip]
const EFFICIENCY: [[i32; StreetFighterType::COUNT]; StreetFighterType::COUNT] = [
    [ 0, 40, 40, 50, 40, 50, 50, 50, 40, 50, 50, 50, 50, 60, 60, 50, 60, 70, 60, 60, 40, 60, 50, 60, 60, 60, 40, 60, 50, 60, 60, 50, 60, 60, 60, 50, 60, 70, 70],
    [60,  0, 60, 60, 50, 50, 60, 40, 50, 40, 60, 50, 60, 40, 60, 60, 50, 40, 70, 60, 50, 60, 50, 40, 60, 60, 50, 60, 50, 50, 50, 60, 50, 50, 60, 60, 50, 60, 60],
    [60, 40,  0, 60, 40, 50, 60, 40, 50, 60, 60, 60, 60, 40, 60, 50, 50, 40, 60, 40, 50, 50, 40, 60, 60, 60, 50, 60, 60, 50, 60, 60, 60, 50, 60, 60, 60, 60, 60],
    [50, 40, 40,  0, 50, 50, 60, 50, 50, 50, 60, 50, 50, 50, 50, 50, 40, 60, 60, 60, 60, 60, 50, 50, 50, 60, 60, 50, 60, 50, 60, 60, 60, 50, 50, 60, 60, 60, 60],
    [60, 50, 60, 50,  0, 50, 60, 40, 50, 50, 50, 60, 50, 50, 60, 50, 50, 60, 60, 50, 50, 60, 60, 60, 40, 50, 50, 50, 50, 50, 60, 50, 50, 50, 60, 50, 60, 60, 60],
    [50, 50, 50, 50, 50,  0, 40, 50, 50, 60, 60, 50, 60, 50, 40, 50, 60, 30, 60, 40, 60, 60, 40, 50, 70, 40, 40, 50, 60, 60, 60, 50, 60, 50, 60, 60, 60, 50, 70],
    [50, 40, 40, 40, 40, 60,  0, 60, 50, 40, 50, 50, 50, 50, 50, 60, 60, 70, 50, 50, 60, 40, 50, 40, 60, 50, 60, 60, 50, 60, 60, 50, 60, 40, 50, 60, 60, 70, 60],
    [50, 60, 60, 50, 60, 50, 40,  0, 60, 50, 50, 60, 60, 50, 50, 50, 50, 40, 40, 40, 60, 40, 50, 50, 60, 50, 60, 50, 50, 60, 50, 50, 50, 60, 60, 50, 50, 50, 60],
    [60, 50, 50, 50, 50, 50, 50, 40,  0, 50, 50, 60, 50, 50, 60, 50, 50, 40, 40, 60, 60, 50, 50, 50, 60, 60, 50, 50, 60, 50, 40, 50, 50, 50, 50, 60, 60, 40, 60],
    [50, 60, 40, 50, 50, 40, 60, 50, 50,  0, 50, 40, 50, 40, 50, 50, 50, 40, 60, 60, 50, 50, 60, 50, 50, 50, 60, 50, 50, 50, 50, 50, 60, 50, 60, 60, 60, 40, 60],
    [50, 40, 40, 40, 50, 40, 50, 50, 50, 50,  0, 60, 50, 50, 60, 50, 50, 40, 70, 60, 50, 40, 50, 40, 50, 60, 50, 50, 50, 50, 60, 60, 60, 50, 60, 60, 60, 40, 50],
    [50, 50, 40, 50, 40, 50, 50, 40, 40, 60, 40,  0, 50, 40, 50, 50, 50, 60, 40, 60, 50, 50, 60, 60, 60, 60, 60, 60, 60, 40, 50, 50, 40, 50, 50, 50, 40, 80, 60],
    [50, 40, 40, 50, 50, 40, 50, 40, 50, 50, 50, 50,  0, 50, 50, 50, 50, 40, 60, 60, 50, 40, 50, 50, 60, 50, 50, 50, 60, 50, 40, 50, 60, 60, 60, 50, 60, 60, 70],
    [40, 60, 60, 50, 50, 50, 50, 50, 50, 60, 50, 60, 50,  0, 50, 50, 40, 40, 50, 30, 60, 50, 60, 60, 50, 50, 50, 50, 60, 50, 40, 50, 50, 50, 50, 50, 60, 50, 60],
    [40, 40, 40, 50, 40, 60, 50, 50, 40, 50, 40, 50, 50, 50,  0, 50, 60, 60, 40, 50, 60, 50, 50, 50, 40, 50, 50, 50, 50, 50, 60, 50, 60, 50, 60, 60, 60, 60, 70],
    [50, 40, 50, 50, 50, 50, 40, 50, 50, 50, 50, 50, 50, 50, 50,  0, 60, 50, 40, 40, 50, 60, 60, 50, 40, 50, 50, 60, 50, 60, 50, 60, 50, 50, 50, 50, 60, 50, 60],
    [40, 50, 50, 60, 50, 40, 40, 50, 50, 50, 50, 50, 50, 60, 40, 40,  0, 40, 70, 60, 50, 60, 50, 50, 60, 40, 50, 50, 50, 50, 60, 60, 60, 50, 50, 50, 50, 40, 60],
    [30, 60, 60, 40, 40, 70, 30, 60, 60, 60, 60, 40, 60, 60, 40, 50, 60,  0, 40, 40, 50, 30, 40, 40, 40, 40, 70, 60, 50, 60, 50, 40, 50, 60, 60, 50, 60, 50, 70],
    [40, 30, 40, 40, 40, 40, 50, 60, 60, 40, 30, 60, 40, 50, 60, 60, 30, 60,  0, 60, 40, 60, 60, 40, 50, 60, 50, 60, 50, 40, 70, 50, 50, 60, 50, 60, 50, 60, 70],
    [40, 40, 60, 40, 50, 60, 50, 60, 40, 40, 40, 40, 40, 70, 50, 60, 40, 60, 40,  0, 50, 60, 50, 60, 40, 50, 40, 50, 40, 50, 60, 50, 50, 60, 50, 50, 60, 70, 60],
    [60, 50, 50, 40, 50, 40, 40, 40, 40, 50, 50, 50, 50, 40, 40, 50, 50, 50, 60, 50,  0, 50, 50, 60, 50, 60, 50, 50, 50, 50, 40, 60, 50, 50, 60, 60, 60, 60, 60],
    [40, 40, 50, 40, 40, 40, 60, 60, 50, 50, 60, 50, 60, 50, 50, 40, 40, 70, 40, 40, 50,  0, 50, 50, 50, 50, 50, 50, 60, 50, 60, 50, 50, 50, 50, 50, 50, 60, 60],
    [50, 50, 60, 50, 40, 60, 50, 50, 50, 40, 50, 40, 50, 40, 50, 40, 50, 60, 40, 50, 50, 50,  0, 50, 40, 50, 50, 50, 50, 60, 60, 50, 50, 50, 50, 50, 50, 70, 60],
    [40, 60, 40, 50, 40, 50, 60, 50, 50, 50, 60, 40, 50, 40, 50, 50, 50, 60, 60, 40, 40, 50, 50,  0, 50, 50, 50, 50, 40, 40, 40, 50, 50, 50, 60, 60, 60, 70, 60],
    [40, 40, 40, 50, 60, 30, 40, 40, 40, 50, 50, 40, 40, 50, 60, 60, 40, 60, 50, 60, 50, 50, 60, 50,  0, 60, 60, 50, 60, 40, 40, 50, 50, 60, 50, 50, 60, 60, 60],
    [40, 40, 40, 40, 50, 60, 50, 50, 40, 50, 40, 40, 50, 50, 50, 50, 60, 60, 40, 50, 40, 50, 50, 50, 40,  0, 40, 40, 60, 60, 60, 50, 50, 60, 60, 60, 50, 60, 60],
    [60, 50, 50, 40, 50, 60, 40, 40, 50, 40, 50, 40, 50, 50, 50, 50, 50, 30, 50, 60, 50, 50, 50, 50, 40, 60,  0, 40, 50, 60, 50, 50, 50, 50, 60, 60, 60, 40, 60],
    [40, 40, 40, 50, 50, 50, 40, 50, 50, 50, 50, 40, 50, 50, 50, 40, 50, 40, 40, 50, 50, 50, 50, 50, 50, 60, 60,  0, 40, 40, 50, 50, 50, 60, 60, 60, 60, 60, 60],
    [50, 50, 40, 40, 50, 40, 50, 50, 40, 50, 50, 40, 40, 40, 50, 50, 50, 50, 50, 60, 50, 40, 50, 60, 40, 40, 50, 60,  0, 60, 50, 50, 50, 50, 60, 60, 50, 60, 50],
    [40, 50, 50, 50, 50, 40, 40, 40, 50, 50, 50, 60, 50, 50, 50, 40, 50, 40, 60, 50, 50, 50, 40, 60, 60, 40, 40, 60, 40,  0, 50, 60, 50, 50, 50, 50, 60, 40, 60],
    [40, 50, 40, 40, 40, 40, 40, 50, 60, 50, 40, 50, 60, 60, 40, 50, 40, 50, 30, 40, 60, 40, 40, 60, 60, 40, 50, 50, 50, 50,  0, 60, 60, 50, 50, 50, 60, 60, 60],
    [50, 40, 40, 40, 50, 50, 50, 50, 50, 50, 40, 50, 50, 50, 50, 40, 40, 60, 50, 50, 40, 50, 50, 50, 50, 50, 50, 50, 50, 40, 40,  0, 50, 50, 50, 50, 50, 70, 60],
    [40, 50, 40, 40, 50, 40, 40, 50, 50, 40, 40, 60, 40, 50, 40, 50, 40, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 40, 50,  0, 50, 50, 50, 60, 60, 60],
    [40, 50, 50, 50, 50, 50, 60, 40, 50, 50, 50, 50, 40, 50, 50, 50, 50, 40, 40, 40, 50, 50, 50, 50, 40, 40, 50, 40, 50, 50, 50, 50, 50,  0, 50, 50, 50, 40, 60],
    [40, 40, 40, 50, 40, 40, 50, 40, 50, 40, 40, 50, 40, 50, 40, 50, 50, 40, 50, 50, 40, 50, 50, 40, 50, 40, 40, 40, 40, 50, 50, 50, 50, 50,  0, 60, 50, 60, 60],
    [50, 40, 40, 40, 50, 40, 40, 50, 40, 40, 40, 50, 50, 50, 40, 50, 50, 50, 40, 50, 40, 50, 50, 40, 50, 40, 40, 40, 40, 50, 50, 50, 50, 50, 40,  0, 50, 60, 60],
    [40, 50, 40, 40, 40, 40, 40, 50, 40, 40, 40, 60, 40, 40, 40, 40, 50, 40, 50, 40, 40, 50, 50, 40, 40, 50, 40, 40, 50, 40, 40, 50, 40, 50, 50, 50,  0, 50, 60],
    [30, 40, 40, 40, 40, 50, 30, 50, 60, 60, 60, 20, 40, 50, 40, 50, 60, 50, 40, 30, 40, 40, 30, 30, 40, 40, 60, 40, 40, 60, 40, 30, 40, 60, 40, 40, 50,  0, 60],
    [30, 40, 40, 40, 40, 30, 40, 40, 40, 40, 50, 40, 30, 40, 30, 40, 40, 30, 30, 40, 40, 40, 40, 40, 40, 40, 40, 40, 50, 40, 40, 40, 40, 40, 40, 40, 40, 40,  0],
];

fn get_effectiveness(attacker: usize, defender: usize) -> i32 {
    EFFICIENCY[attacker][defender]
}

#[derive(Clone)]
pub struct StreetFighter {
    health: i32,
    damage: i32,
    kind: StreetFighterType,
    rng: rand::rngs::ThreadRng,
}

impl StreetFighter {
    pub fn new(kind: StreetFighterType) -> Self {
        StreetFighter {
            health: 100,
            damage: 100,
            kind,
            rng: rand::thread_rng(),
        }
    }

    fn reset(&mut self, kind: StreetFighterType) {
        self.health = 100;
        self.damage = 100;
        self.kind = kind;
    }
}

impl Fighter for StreetFighter {
    fn should_fight(&self, defender: &Self) -> bool {
        self.kind != defender.kind
    }

    fn get_effectiveness(&self, defender: &Self) -> i32 {
        get_effectiveness(self.kind.into(), defender.kind.into())
    }

    fn fight(&self, defender: &mut Self) -> bool {
        // street fighter table is based on chance to win, not damage done
        // so we need to roll a die based on the chance
        // we use defender's rng because they're &mut Self
        let roll = defender.rng.gen_range(0..=100);
        let effectiveness = self.get_effectiveness(defender);
        if roll < effectiveness {
            defender.reset(self.kind);
            true
        } else {
            false
        }
    }
}

impl RandomlyGeneratable for StreetFighter {
    fn generate_randomly() -> Box<dyn Iterator<Item = Self>> {
        let rng = rand::thread_rng();
        Box::new(
            rng.sample_iter(Uniform::from(0..StreetFighterType::COUNT))
                .map(|t| Self::new(t.into())),
        )
    }
}

impl Colored for StreetFighter {
    fn color(&self) -> nannou::image::Rgb<u8> {
        self.kind.into()
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    // todo, tests

    #[test]
    fn test_get_effectiveness() {}

    #[test]
    fn test_get_color() {}

    #[test]
    fn test_damage() {}

    #[test]
    fn test_reset() {}
}
