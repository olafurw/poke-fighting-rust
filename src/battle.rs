use rand::distributions::Uniform;
use rand::Rng;

use crate::{Pokemon, POKEMON_COUNT, get_effectiveness_with_type};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Location
{
    pub x: usize,
    pub y: usize
}

impl Location
{
    pub fn is_outside(&self, img_width: usize, img_height: usize) -> bool
    {
        self.x > img_width || self.y > img_height
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SelectionAlgorithm
{
    WeakestNeighbour,
    RandomNeighbour,
}

pub struct Battle
{
    pokemons: Vec<Vec<Pokemon>>,
    rng: rand::rngs::ThreadRng,
    selection_algorithm: SelectionAlgorithm,
}

impl Battle
{
    pub fn new(img_width: usize, img_height: usize, selection_algorithm: SelectionAlgorithm) -> Self
    {
        let die = Uniform::from(0 .. POKEMON_COUNT);

        let mut battle = Battle { pokemons: Vec::with_capacity(img_height), rng: rand::thread_rng(), selection_algorithm };
        for _ in 0 .. img_height
        {
            let row = (0 .. img_width).map(|_| Pokemon::random(&mut battle.rng, &die)).collect();
            battle.pokemons.push(row);
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
        let img_width = self.pokemons[0].len();
        let img_height = self.pokemons.len();
        let num_entries = img_width * img_height;

        let mut death_count = 0;
        let start = self.rng.gen_range(0 .. num_entries);
        let offset = PRIMES[self.rng.gen_range(0 .. PRIMES.len())];
        let mut current = start;

        loop
        {
            let attacker_loc = Location { x: current % img_width, y: current / img_width };
            let defender_loc = match self.selection_algorithm
            {
                SelectionAlgorithm::WeakestNeighbour => self.weakest_neighbour(attacker_loc),
                SelectionAlgorithm::RandomNeighbour => self.random_neighbour(attacker_loc)
            };

            if self.fight(attacker_loc, defender_loc)
            {
                death_count += 1;
            }

            current = (current + offset) % num_entries;
            if current == start
            {
                break;
            }
        }

        death_count
    }

    pub fn fight(&mut self, attacker_loc: Location, defender_loc: Location) -> bool
    {
        let img_width = self.pokemons[0].len();
        let img_height = self.pokemons.len();
        if attacker_loc == defender_loc || attacker_loc.is_outside(img_width, img_height) || defender_loc.is_outside(img_width, img_height)
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

    pub fn weakest_neighbour(&self, origin: Location) -> Location
    {
        let img_width = self.pokemons[0].len();
        let img_height = self.pokemons.len();
        if origin.is_outside(img_width, img_height)
        {
            return Location { x: 0, y: 0 };
        }

        let pokemon = &self.pokemons[origin.y][origin.x];

        let candidates = [
            Location { x: origin.x, y: (origin.y + img_height - 1) % img_height },
            Location { x: (origin.x + 1) % img_width, y: origin.y },
            Location { x: origin.x, y: (origin.y + 1) % img_height },
            Location { x: (origin.x + img_width - 1) % img_width, y: origin.y },
        ];
        *candidates.iter().max_by_key(|candidate|
        {
            let neighbour = &self.pokemons[candidate.y][candidate.x];
            get_effectiveness_with_type(pokemon.kind, neighbour.kind)
        }).unwrap()
    }

    pub fn random_neighbour(&mut self, origin: Location) -> Location
    {
        let img_width = self.pokemons[0].len();
        let img_height = self.pokemons.len();
        if origin.is_outside(img_width, img_height)
        {
            return Location { x: 0, y: 0 };
        }

        let direction = self.rng.gen_range(0 .. 4);
        if direction == 0 // Go up
        {
            Location { x: origin.x, y: (origin.y + img_height - 1) % img_height }
        }
        else if direction == 1 // Go right
        {
            Location { x: (origin.x + 1) % img_width , y: origin.y }
        }
        else if direction == 2 // Go down
        {
            Location { x: origin.x, y: (origin.y + 1) % img_height }
        }
        else // Go left
        {
            Location { x: (origin.x + img_width - 1) % img_width, y: origin.y }
        }
    }
}
