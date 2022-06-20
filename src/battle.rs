use crate::grid::{Grid2D, Size};
use crate::types::GenerateRandomly;
use rand::seq::IteratorRandom;
use rand::Rng;

type Location = (usize, usize);

#[derive(Debug, Copy, Clone)]
pub enum SelectionAlgorithm {
    WeakestNeighbour,
    RandomNeighbour,
}

pub trait Fighter {
    fn should_fight(&self, defender: &Self) -> bool;
    fn get_effectiveness(&self, defender: &Self) -> i32;
    fn fight(&self, defender: &mut Self) -> bool;
}

pub struct Battle<T> {
    fighters: Grid2D<T>,
    rng: rand::rngs::ThreadRng,
    selection_callback: fn(&mut Self, Location, Size) -> Option<Location>,
}

impl<T> Battle<T>
where
    T: GenerateRandomly + Fighter,
{
    pub fn new(
        img_width: usize,
        img_height: usize,
        selection_algorithm: SelectionAlgorithm,
        filter_fight_candidates: bool,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let fighters = Grid2D::new_with((img_width, img_height), || T::generate_randomly(&mut rng));

        Self {
            fighters,
            rng,
            selection_callback: if filter_fight_candidates {
                match selection_algorithm {
                    SelectionAlgorithm::WeakestNeighbour => Battle::weakest_neighbour_filtered,
                    SelectionAlgorithm::RandomNeighbour => Battle::random_neighbour_filtered,
                }
            } else {
                match selection_algorithm {
                    SelectionAlgorithm::WeakestNeighbour => Battle::weakest_neighbour,
                    SelectionAlgorithm::RandomNeighbour => Battle::random_neighbour,
                }
            },
        }
    }
}

impl<T> Battle<T>
where
    T: Fighter,
{
    pub fn fighter(&self, location: Location) -> Option<&T> {
        self.fighters.get(location)
    }

    pub fn action(&mut self) {
        // We use prime numbers as offsets to loop through the entries in a semi-random fashion.
        // These particular prime numbers have been chosen by a fair dice roll.
        const PRIMES: &[usize] = &[48817, 58099, 89867, 105407, 126943, 200723, 221021, 231677];
        let (w, h) = self.fighters.size();
        let num_entries = self.fighters.count();

        let start = self.rng.gen_range(0..num_entries);
        let offset = PRIMES[self.rng.gen_range(0..PRIMES.len())];
        let mut current = start;

        loop {
            let attacker_loc = (current % w, current / w);
            let defender_loc = (self.selection_callback)(self, attacker_loc, (w, h));
            if let Some(defender_loc) = defender_loc {
                self.fight(attacker_loc, defender_loc);
            }

            current = (current + offset) % num_entries;
            if current == start {
                break;
            }
        }
    }

    pub fn fight(&mut self, attacker_loc: Location, defender_loc: Location) {
        if let Some((attacker, defender)) = self.fighters.get_pair_mut(attacker_loc, defender_loc) {
            attacker.fight(defender);
        }
    }

    fn weakest_neighbour(&mut self, origin: Location, size: Size) -> Option<Location> {
        let fighter = self.fighters.get(origin)?;
        neighbours(origin, size)
            .into_iter()
            .filter_map(|candidate| get_candidate(&self.fighters, candidate))
            .max_by_key(|(neighbour, _)| fighter.get_effectiveness(neighbour))
            .map(|(_, candidate)| candidate)
    }

    fn weakest_neighbour_filtered(&mut self, origin: Location, size: Size) -> Option<Location> {
        let fighter = self.fighters.get(origin)?;
        neighbours(origin, size)
            .into_iter()
            .into_iter()
            .filter_map(|candidate| get_candidate(&self.fighters, candidate))
            .filter(|(neighbour, _)| fighter.should_fight(neighbour))
            .max_by_key(|(neighbour, _)| fighter.get_effectiveness(neighbour))
            .map(|(_, candidate)| candidate)
    }

    fn random_neighbour(&mut self, origin: Location, size: Size) -> Option<Location> {
        neighbours(origin, size).into_iter().choose(&mut self.rng)
    }

    fn random_neighbour_filtered(&mut self, origin: Location, size: Size) -> Option<Location> {
        let fighter = self.fighters.get(origin)?;
        neighbours(origin, size)
            .into_iter()
            .filter_map(|candidate| get_candidate(&self.fighters, candidate))
            .filter(|(neighbour, _)| fighter.should_fight(neighbour))
            .choose(&mut self.rng)
            .map(|(_, candidate)| candidate)
    }
}

pub fn neighbours((x, y): Location, (w, h): Size) -> [Location; 4] {
    [
        (x, (y + h - 1) % h),
        ((x + 1) % w, y),
        (x, (y + 1) % h),
        ((x + w - 1) % w, y),
    ]
}

fn get_candidate<T>(grid: &Grid2D<T>, location: Location) -> Option<(&T, Location)> {
    let item = grid.get(location)?;
    Some((item, location))
}
