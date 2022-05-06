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
        &self.pokemons[y as usize][x as usize]
    }

    pub fn action(&mut self) -> u32
    {
        // We use prime numbers as offsets to loop through the entries in a semi-random fashion.
        // These particular prime numbers have been chosen by a fair dice roll.
        const PRIMES: &[usize] = &[48817, 58099, 89867, 105407, 126943, 200723, 221021, 231677];
        const NUM_ENTRIES: usize = IMG_SIZE * IMG_SIZE;

        let mut death_count = 0;
        let start = self.rng.gen_range(0 .. NUM_ENTRIES);
        let offset = PRIMES[self.rng.gen_range(0 .. PRIMES.len())];
        let mut current = start;

        loop
        {
            let attacker_loc = Location { x: current % IMG_SIZE, y: current / IMG_SIZE };
            let defender_loc = self._weakest_neighbour(attacker_loc);
            //let defender_loc = self._random_neighbour(attacker_loc);

            if self.fight(attacker_loc, defender_loc)
            {
                death_count += 1;
            }

            current = (current + offset) % NUM_ENTRIES;
            if current == start
            {
                break;
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
        let damage = attacker_damage * effectiveness / 100;

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
        if origin.is_outside()
        {
            return Location { x: 0, y: 0 };
        }

        let pokemon = &self.pokemons[origin.y][origin.x];

        let candidates = [
            Location { x: origin.x, y: (origin.y + IMG_SIZE - 1) % IMG_SIZE },
            Location { x: (origin.x + 1) % IMG_SIZE, y: origin.y },
            Location { x: origin.x, y: (origin.y + 1) % IMG_SIZE },
            Location { x: (origin.x + IMG_SIZE - 1) % IMG_SIZE, y: origin.y },
        ];
        *candidates.iter().max_by_key(|candidate|
        {
            let neighbour = &self.pokemons[candidate.y][candidate.x];
            get_effectiveness_with_type(pokemon.kind, neighbour.kind)
        }).unwrap()
    }

    pub fn _random_neighbour(&mut self, origin: Location) -> Location
    {
        if origin.is_outside()
        {
            return Location { x: 0, y: 0 };
        }

        let direction = self.rng.gen_range(0 .. 4);
        if direction == 0 // Go up
        {
            Location { x: origin.x, y: (origin.y + IMG_SIZE - 1) % IMG_SIZE }
        }
        else if direction == 1 // Go right
        {
            Location { x: (origin.x + 1) % IMG_SIZE , y: origin.y }
        }
        else if direction == 2 // Go down
        {
            Location { x: origin.x, y: (origin.y + 1) % IMG_SIZE }
        }
        else // Go left
        {
            Location { x: (origin.x + IMG_SIZE - 1) % IMG_SIZE, y: origin.y }
        }
    }
}
