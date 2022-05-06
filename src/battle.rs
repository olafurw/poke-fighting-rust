use rand::distributions::Uniform;
use rand::Rng;

use crate::{Pokemon, IMG_SIZE, POKEMON_COUNT, get_effectiveness_with_type};

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
        self.x > IMG_SIZE || self.y > IMG_SIZE
    }
}

pub struct Battle
{
    pokemons: Vec<Vec<Pokemon>>,
    rng: rand::rngs::ThreadRng,
}

impl Battle
{
    pub fn new() -> Self
    {
        let die = Uniform::from(0 .. POKEMON_COUNT);

        let mut battle = Battle { pokemons: Vec::with_capacity(IMG_SIZE), rng: rand::thread_rng() };
        for _ in 0 .. IMG_SIZE
        {
            let row = [(); IMG_SIZE].map(|_| Pokemon::random(&mut battle.rng, &die));
            battle.pokemons.push(Vec::from(row));
        }

        battle
    }

    pub fn pokemon(&self, x: u32, y: u32) -> &Pokemon
    {
        return &self.pokemons[y as usize][x as usize];
    }

    pub fn action(&mut self) -> u32
    {
        let mut death_count = 0;

        for x in 0..IMG_SIZE
        {
            for y in 0..IMG_SIZE
            {
                let attacker_loc = Location { x, y };
                //let defender_loc = self._weakest_neighbour(attacker_loc);
                let defender_loc = self._random_neighbour(attacker_loc);

                if self.fight(attacker_loc, defender_loc)
                {
                    death_count += 1;
                }
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
            defender.reset(attacker_kind);
            true
        }
        else
        {
            false
        }
    }

    pub fn _weakest_neighbour(&self, origin: Location) -> Location
    {
        let mut highest_effectiveness: f32 = 0.0;
        let mut location = Location { x: 0, y: 0 };
        if origin.is_outside()
        {
            return location;
        }

        let pokemon = &self.pokemons[origin.y][origin.x];

        // waage todo: refactor these 4, they do similar things
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
        if origin.x != IMG_SIZE - 1 // there is a right neighbour
        {
            let neighbour = &self.pokemons[origin.y][origin.x + 1];
            let effectiveness = get_effectiveness_with_type(pokemon.kind, neighbour.kind);
            if effectiveness > highest_effectiveness
            {
                highest_effectiveness = effectiveness;
                location = Location { x: origin.x + 1, y: origin.y };
            }
        }
        if origin.y != IMG_SIZE - 1 // there is a bottom neighbour
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

    pub fn _random_neighbour(&mut self, origin: Location) -> Location
    {
        let location = Location { x: 0, y: 0 };
        if origin.is_outside()
        {
            return location;
        }

        let direction = self.rng.gen_range(0 .. 4);
        if direction == 0       // Go up
        {
            Location { x: origin.x, y: (origin.y + IMG_SIZE - 1) % IMG_SIZE }
        }
        else if direction == 1  // Go right
        {
            Location { x: (origin.x + 1) % IMG_SIZE , y: origin.y }
        }
        else if direction == 2  // Go down
        {
            Location { x: origin.x, y: (origin.y + 1) % IMG_SIZE }
        }
        else                    // Go left
        {
            Location { x: (origin.x + IMG_SIZE - 1) % IMG_SIZE , y: origin.y }
        }
    }
}
