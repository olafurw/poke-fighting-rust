use rand::Rng;
use rand::seq::IteratorRandom;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl Location {
    pub fn is_outside(&self, img_width: usize, img_height: usize) -> bool {
        self.x > img_width || self.y > img_height
    }

    pub fn neighbours(&self, img_width: usize, img_height: usize) -> [Location; 4] {
        [
            Location {
                x: self.x,
                y: (self.y + img_height - 1) % img_height,
            },
            Location {
                x: (self.x + 1) % img_width,
                y: self.y,
            },
            Location {
                x: self.x,
                y: (self.y + 1) % img_height,
            },
            Location {
                x: (self.x + img_width - 1) % img_width,
                y: self.y,
            },
        ]
    }
}

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
    fighters: Vec<Vec<T>>,
    rng: rand::rngs::ThreadRng,
    selection_callback: fn(&mut Self, Location, usize, usize) -> Location,
}

impl<T: Fighter> Battle<T> {
    pub fn new(
        mut fighter_source: impl Iterator<Item = T>,
        img_width: usize,
        img_height: usize,
        selection_algorithm: SelectionAlgorithm,
        filter_fight_candidates: bool,
    ) -> Self {
        let mut battle = Self {
            fighters: Vec::with_capacity(img_height),
            rng: rand::thread_rng(),
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
        };
        for _ in 0..img_height {
            let row = (0..img_width)
                .map(|_| fighter_source.next().unwrap())
                .collect();
            battle.fighters.push(row);
        }

        battle
    }

    pub fn fighter(&self, x: u32, y: u32) -> &T {
        &self.fighters[y as usize][x as usize]
    }

    fn fighters(&mut self, loc1: Location, loc2: Location) -> (&mut T, &mut T) {
        // Best way to get two mutable references to one array seems to be split_at_mut().
        // It's rather awkward for a two-dimensional array however.
        if loc1.y == loc2.y {
            let (slice1, slice2) =
                self.fighters[loc1.y].split_at_mut(std::cmp::max(loc1.x, loc2.x));
            if loc1.x < loc2.x {
                (&mut slice1[loc1.x], &mut slice2[0])
            } else {
                (&mut slice2[0], &mut slice1[loc2.x])
            }
        } else {
            let (slice1, slice2) = self.fighters.split_at_mut(std::cmp::max(loc1.y, loc2.y));
            if loc1.y < loc2.y {
                (&mut slice1[loc1.y][loc1.x], &mut slice2[0][loc2.x])
            } else {
                (&mut slice2[0][loc1.x], &mut slice1[loc2.y][loc2.x])
            }
        }
    }

    pub fn action(&mut self) -> u32 {
        // We use prime numbers as offsets to loop through the entries in a semi-random fashion.
        // These particular prime numbers have been chosen by a fair dice roll.
        const PRIMES: &[usize] = &[48817, 58099, 89867, 105407, 126943, 200723, 221021, 231677];
        let img_width = self.fighters[0].len();
        let img_height = self.fighters.len();
        let num_entries = img_width * img_height;

        let mut death_count = 0;
        let start = self.rng.gen_range(0..num_entries);
        let offset = PRIMES[self.rng.gen_range(0..PRIMES.len())];
        let mut current = start;

        loop {
            let attacker_loc = Location {
                x: current % img_width,
                y: current / img_width,
            };
            let defender_loc = (self.selection_callback)(self, attacker_loc, img_width, img_height);

            if self.fight(attacker_loc, defender_loc) {
                death_count += 1;
            }

            current = (current + offset) % num_entries;
            if current == start {
                break;
            }
        }

        death_count
    }

    pub fn fight(&mut self, attacker_loc: Location, defender_loc: Location) -> bool {
        let img_width = self.fighters[0].len();
        let img_height = self.fighters.len();
        if attacker_loc == defender_loc
            || attacker_loc.is_outside(img_width, img_height)
            || defender_loc.is_outside(img_width, img_height)
        {
            return false;
        }

        let (attacker, defender) = self.fighters(attacker_loc, defender_loc);
        attacker.fight(defender)
    }

    fn weakest_neighbour(
        &mut self,
        origin: Location,
        img_width: usize,
        img_height: usize
    ) -> Location {
        let fighter = &self.fighters[origin.y][origin.x];

        *origin.neighbours(img_width, img_height).iter().max_by_key(|candidate| {
            let neighbour = &self.fighters[candidate.y][candidate.x];
            fighter.get_effectiveness(neighbour)
        }).unwrap_or(&origin)
    }

    fn weakest_neighbour_filtered(
        &mut self,
        origin: Location,
        img_width: usize,
        img_height: usize
    ) -> Location {
        let fighter = &self.fighters[origin.y][origin.x];

        *origin.neighbours(img_width, img_height).iter().filter(|candidate| {
            let neighbour = &self.fighters[candidate.y][candidate.x];
            fighter.should_fight(neighbour)
        }).max_by_key(|candidate| {
            let neighbour = &self.fighters[candidate.y][candidate.x];
            fighter.get_effectiveness(neighbour)
        }).unwrap_or(&origin)
    }

    fn random_neighbour(
        &mut self,
        origin: Location,
        img_width: usize,
        img_height: usize
    ) -> Location {
        *origin.neighbours(img_width, img_height).iter().choose(&mut self.rng).unwrap_or(&origin)
    }

    fn random_neighbour_filtered(
        &mut self,
        origin: Location,
        img_width: usize,
        img_height: usize
    ) -> Location {
        let fighter = &self.fighters[origin.y][origin.x];

        *origin.neighbours(img_width, img_height).iter().filter(|candidate| {
            let neighbour = &self.fighters[candidate.y][candidate.x];
            fighter.should_fight(neighbour)
        }).choose(&mut self.rng).unwrap_or(&origin)
    }
}
