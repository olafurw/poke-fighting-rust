use crate::battle::Fighter;
use crate::types::{Colored, GenerateRandomly};
use rand::Rng;

#[derive(Clone)]
pub struct ColorFighter {
    color: [u8; 3],
}

impl Default for ColorFighter {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorFighter {
    pub fn new() -> Self {
        ColorFighter {
            color: [0, 0, 0],
        }
    }

    fn reset(&mut self, color: &[u8; 3]) {
        self.color = *color;
    }

    fn take_damage(&mut self, color: &[u8; 3]) -> bool {
        for (i, c) in self.color.iter_mut().enumerate() {
            if color[i] < *c {
                *c -= color[i];
            } else {
                *c = 0;
            }
        }

        self.color.iter().any(|c| *c == 0)
    }
}

impl Fighter for ColorFighter {
    fn should_fight(&self, defender: &Self) -> bool {
        for (i, c) in self.color.iter().enumerate() {
            if *c >= defender.color[i] {
                return true;
            }
        }
        
        false
    }

    fn get_effectiveness(&self, _defender: &Self) -> i32 {
        1
    }

    fn fight(&self, defender: &mut Self) -> bool {
        let _effectiveness = self.get_effectiveness(defender);

        let is_dead = defender.take_damage(&self.color);
        if is_dead {
            defender.reset(&self.color);
        }
        is_dead
    }
}

impl GenerateRandomly for ColorFighter {
    fn generate_randomly<R>(rng: &mut R) -> Self
    where
        R: Rng,
    {
        let mut s = Self::new();
        s.color[0] = rng.gen_range(0..=255);
        s.color[1] = rng.gen_range(0..=255);
        s.color[2] = rng.gen_range(0..=255);
        
        s
    }
}

impl Colored for ColorFighter {
    fn color(&self) -> nannou::image::Rgb<u8> {
        self.color.into()
    }
}

impl core::fmt::Display for ColorFighter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.color)
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn convert_type_from_usize() {
    }

    #[test]
    fn test_get_effectiveness() {
    }

    #[test]
    fn test_get_color() {
    }

    #[test]
    fn test_damage() {
    }

    #[test]
    fn test_reset() {
    }
}
