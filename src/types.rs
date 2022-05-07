use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
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

impl PokemonType
{
	pub fn random(rng: &mut ThreadRng, die: &Uniform<usize>) -> Self
	{
		let value = die.sample(rng);
		PokemonType::try_from(value).unwrap()
	}
}

impl From<PokemonType> for [u8; 3]
{
	fn from(kind: PokemonType) -> Self
	{
		match kind
		{
			PokemonType::Normal => [168,168,120u8],
			PokemonType::Fire => [240,128,48u8],
			PokemonType::Water => [104,144,240u8],
			PokemonType::Electric => [248,208,48u8],
			PokemonType::Grass => [120,200,80u8],
			PokemonType::Ice => [152,216,216u8],
			PokemonType::Fighting => [192,48,40u8],
			PokemonType::Poison => [160,64,160u8],
			PokemonType::Ground => [224,192,104u8],
			PokemonType::Flying => [168,144,240u8],
			PokemonType::Psychic => [248,88,136u8],
			PokemonType::Bug => [168,184,32u8],
			PokemonType::Rock => [184,160,56u8],
			PokemonType::Ghost => [112,88,152u8],
			PokemonType::Dragon => [112,56,248u8],
			PokemonType::Dark => [112,88,72u8],
			PokemonType::Steel => [184,184,208u8],
			PokemonType::Fairy => [240,182,188u8],
		}
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{PokemonType};

    #[test]
    fn convert_type_from_usize()
    {
        assert_eq!(PokemonType::from(0), PokemonType::Normal);
        assert_eq!(PokemonType::from(17), PokemonType::Fairy);
    }
}
