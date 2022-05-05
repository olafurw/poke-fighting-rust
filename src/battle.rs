use rand::prelude::ThreadRng;
use rand::distributions::Uniform;

use crate::{Pokemon, POKEMON_IMG_SIZE, get_effectiveness_with_type};

#[derive(PartialEq, Copy, Clone)]
pub struct Location
{
    pub x: usize,
    pub y: usize
}

impl Location
{
    pub fn is_outside(&self) -> bool
    {
        self.x > POKEMON_IMG_SIZE || self.y > POKEMON_IMG_SIZE
    }
}

pub struct Battle
{
    pub pokemons: Vec<Vec<Pokemon>>,
}

impl Battle
{
    pub fn new(rng: &mut ThreadRng, die: &Uniform<usize>) -> Self
    {
        let mut battle = Battle { pokemons: Vec::with_capacity(POKEMON_IMG_SIZE) };
        for _ in 0 .. POKEMON_IMG_SIZE
        {
            let row = [(); POKEMON_IMG_SIZE].map(|_| Pokemon::random(rng, die));
            battle.pokemons.push(Vec::from(row));
        }

        battle
    }

    pub fn action(&mut self) -> u32
    {
        let mut death_count = 0;

        let action_count = POKEMON_IMG_SIZE * POKEMON_IMG_SIZE;
        for n in 0..action_count
        {
            let x = n % POKEMON_IMG_SIZE;
            let y = (n as f32 / POKEMON_IMG_SIZE as f32) as usize;

            let attacker_loc = Location { x, y };
            let defender_loc = self.weakest_neighbour(attacker_loc);
            
            if self.fight(attacker_loc, defender_loc)
            {
                death_count += 1;
            }
        }

        death_count
    }

    pub fn fight(&mut self, attacker_loc: Location, defender_loc: Location) -> bool
    {
        if attacker_loc == defender_loc || attacker_loc.is_outside() || defender_loc.is_outside()
        {
            return false;
        }

        let attacker_kind = self.pokemons[attacker_loc.y][attacker_loc.x].kind;
        let attacker_damage = self.pokemons[attacker_loc.y][attacker_loc.x].damage;
        let defender = &mut self.pokemons[defender_loc.y][defender_loc.x];

        let effectiveness = get_effectiveness_with_type(attacker_kind, defender.kind);
        let damage: i32 = (attacker_damage as f32 * effectiveness) as i32;

        let is_dead = defender.take_damage(damage);
        if is_dead
        {
            *defender = Pokemon::new(attacker_kind);
            true
        }
        else
        {
            false
        }
    }

    pub fn weakest_neighbour(&self, origin: Location) -> Location
    {
        let mut highest_effectiveness: f32 = 0.0;
        let mut location = Location { x: 0, y: 0 };
        if origin.is_outside()
        {
            return location;
        }

        let pokemon = &self.pokemons[origin.y][origin.x];

        if origin.y != 0 // there is a top neighbour
        {
            let neighbour = &self.pokemons[origin.y - 1][origin.x];
            let effectiveness = get_effectiveness_with_type(pokemon.kind, neighbour.kind);
            if effectiveness > highest_effectiveness
            {
                highest_effectiveness = effectiveness;
                location = Location { x: origin.x, y: origin.y - 1 };
            }
        }
        if origin.x != POKEMON_IMG_SIZE - 1 // there is a right neighbour
        {
            let neighbour = &self.pokemons[origin.y][origin.x + 1];
            let effectiveness = get_effectiveness_with_type(pokemon.kind, neighbour.kind);
            if effectiveness > highest_effectiveness
            {
                highest_effectiveness = effectiveness;
                location = Location { x: origin.x + 1, y: origin.y };
            }
        }
        if origin.y != POKEMON_IMG_SIZE - 1 // there is a bottom neighbour
        {
            let neighbour = &self.pokemons[origin.y + 1][origin.x];
            let effectiveness = get_effectiveness_with_type(pokemon.kind, neighbour.kind);
            if effectiveness > highest_effectiveness
            {
                highest_effectiveness = effectiveness;
                location = Location { x: origin.x, y: origin.y + 1 };
            }
        }
        if origin.x != 0 // there is a left neighbour
        {
            let neighbour = &self.pokemons[origin.y][origin.x - 1];
            let effectiveness = get_effectiveness_with_type(pokemon.kind, neighbour.kind);
            if effectiveness > highest_effectiveness
            {
                //highest_effectiveness = effectiveness;
                location = Location { x: origin.x - 1, y: origin.y };
            }
        }

        location
    }
}