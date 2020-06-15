use std::collections::HashSet;

#[derive(Clone)]
struct Moon {
    position: Position,
    velocity: Velocity,
}

impl Moon {
    fn new(position: Position) -> Self {
        Self {
            position,
            velocity: Velocity(0, 0, 0),
        }
    }

    fn apply_gravity(&mut self, moon: &Moon) {
        // Update x
        self.velocity.0 += match self.position.0.cmp(&moon.position.0) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };

        // Update y
        self.velocity.1 += match self.position.1.cmp(&moon.position.1) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };

        // Update z
        self.velocity.2 += match self.position.2.cmp(&moon.position.2) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
    }

    fn apply_velocity(&mut self) {
        // Update x
        self.position.0 += self.velocity.0;

        // Update y
        self.position.1 += self.velocity.1;

        // Update z
        self.position.2 += self.velocity.2;
    }

    fn calculate_total_energy(&self) -> i64 {
        let potential_energy = self.position.get_potential_energy();
        let kinectic_energy = self.velocity.get_kinetic_energy();
        
        potential_energy * kinectic_energy
    }
}

#[derive(Clone)]
pub struct Position(i64, i64, i64);

impl Position {
    pub fn from(data: &str) -> Self {
        let position: Vec<i64> = data
            .split(',')
            .map(|x| {
                x.split('=')
                    .nth(1)
                    .expect("Invalid input")
                    .replace('>', "")
                    .parse()
                    .expect("Invalid number")
            })
            .collect();
        Self(position[0], position[1], position[2])
    }

    fn get_potential_energy(&self) -> i64 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}
#[derive(Clone)]
struct Velocity(i64, i64, i64);

impl Velocity {
    fn get_kinetic_energy(&self) -> i64 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

pub struct Space {
    moons: Vec<Moon>,
}

impl Default for Space {
    fn default() -> Self {
        Space::new()
    }
}

impl Space {
    pub fn new() -> Self {
        Self { moons: vec![] }
    }

    pub fn add_moon(&mut self, position: Position) {
        let moon = Moon::new(position);
        self.moons.push(moon);
    }

    fn update_gravity(&mut self) {
        // Generate pairs
        let mut moon_pairs = HashSet::new();
        for i in 0..self.moons.len() {
            for j in 0..self.moons.len() {
                if i != j && !moon_pairs.contains(&(j, i)) {
                    moon_pairs.insert((i, j));
                }
            }
        }

        let moons_clone = self.moons.clone();
        for (i, j) in moon_pairs.into_iter() {
            self.moons[i].apply_gravity(&moons_clone[j]);
            self.moons[j].apply_gravity(&moons_clone[i]);
        }
    }

    fn update_velocity(&mut self) {
        self.moons.iter_mut().for_each(|x| x.apply_velocity());
    }

    fn run_step(&mut self) {
        self.update_gravity();
        self.update_velocity()
    }

    pub fn run_steps(&mut self, n_steps: usize) {
        for _ in 1..=n_steps {
            self.run_step()
        }
    }

    pub fn get_total_energy(&self) -> i64 {
        self.moons.iter().map(|x| x.calculate_total_energy()).sum()
    }

    pub fn debug(&self) {
        for moon in &self.moons {
            println!(
                "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>",
                moon.position.0,
                moon.position.1,
                moon.position.2,
                moon.velocity.0,
                moon.velocity.1,
                moon.velocity.2
            );
        }
        println!();
    }
}
