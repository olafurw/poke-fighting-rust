use nannou::image::Rgb;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;
use strum::{FromRepr,EnumCount};

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumCount, FromRepr)]
#[repr(usize)]
pub enum PokemonType
{
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
	Fairy
}

pub const POKEMON_COUNT: usize = PokemonType::COUNT;

impl PokemonType
{
	pub fn random(rng: &mut ThreadRng, die: &Uniform<usize>) -> Self
	{
		let value = die.sample(rng);
		PokemonType::from_repr(value).unwrap()
	}
}

impl From<PokemonType> for usize
{
    fn from(kind: PokemonType) -> Self
    {
        kind as Self
    }
}

impl From<PokemonType> for Rgb<u8>
{
	fn from(kind: PokemonType) -> Self
	{
		match kind
		{
			PokemonType::Normal => [168,168,120],
			PokemonType::Fire => [240,128,48],
			PokemonType::Water => [104,144,240],
			PokemonType::Electric => [248,208,48],
			PokemonType::Grass => [120,200,80],
			PokemonType::Ice => [152,216,216],
			PokemonType::Fighting => [192,48,40],
			PokemonType::Poison => [160,64,160],
			PokemonType::Ground => [224,192,104],
			PokemonType::Flying => [168,144,240],
			PokemonType::Psychic => [248,88,136],
			PokemonType::Bug => [168,184,32],
			PokemonType::Rock => [184,160,56],
			PokemonType::Ghost => [112,88,152],
			PokemonType::Dragon => [112,56,248],
			PokemonType::Dark => [112,88,72],
			PokemonType::Steel => [184,184,208],
			PokemonType::Fairy => [240,182,188],
		}.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{PokemonType};

    #[test]
    fn convert_type_from_usize()
    {
        assert_eq!(PokemonType::from_repr(0).unwrap(), PokemonType::Normal);
        assert_eq!(PokemonType::from_repr(17).unwrap(), PokemonType::Fairy);
    }
}
