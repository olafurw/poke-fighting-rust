use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;

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

pub fn pokemontype_from_usize(value: usize) -> PokemonType
{
    PokemonType::try_from(value).unwrap_or_else(|_| panic!("Invalid Pokemon Value: {}", value))
}

pub fn get_random_type(rng: &mut ThreadRng, die: &Uniform<usize>) -> PokemonType
{
    let value = die.sample(rng);
    pokemontype_from_usize(value)
}

#[cfg(test)]
mod tests {
    use crate::types::{PokemonType, pokemontype_from_usize};

    #[test]
    fn convert_type_from_usize()
    {
        assert_eq!(pokemontype_from_usize(0), PokemonType::Normal);
        assert_eq!(pokemontype_from_usize(17), PokemonType::Fairy);
    }
}