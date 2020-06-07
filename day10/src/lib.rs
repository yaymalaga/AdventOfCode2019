use std::cmp::Ordering;

// The line equation is A*x + B*y + C = 0
//
// The point-slope equation is y - y1 = m * (x - x1)
// where m is (y2 - y1) / (x2 - x1)
//
// If we solve for B = 1 we get:
// A = dy / dx
// B = 1
// C = (x1 * y2 - y1 * x2) / dx
#[derive(Debug)]
struct Line {
    dx: i32,
    dy: i32,
    c_integral: i32,
}

impl Line {
    fn from(point_1: Point, point_2: Point) -> Self {
        let dx = point_2.0 as i32 - point_1.0 as i32;
        let dy = point_2.1 as i32 - point_1.1 as i32;
        let c_integral = (point_1.0 * point_2.1) as i32 - (point_1.1 * point_2.0) as i32;

        Self { dx, dy, c_integral }
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        // Identical line
        if self.dx == other.dx && self.dy == other.dy && self.c_integral == other.c_integral {
            return true;
        }

        // Same Y (y2 - y1)
        if self.dy == 0 && self.dy == other.dy {
            return true;
        }

        // Same X (x2 - x1)
        if self.dx == 0 && self.dx == other.dx {
            return true;
        }

        // Proportional line
        let self_a = self.dx as f64 / self.dy as f64;
        let self_c = self.c_integral as f64 / self.dx as f64;

        let other_a = other.dx as f64 / other.dy as f64;
        let other_c = other.c_integral as f64 / other.dx as f64;

        self_a == other_a && self_c == other_c
    }
}

// Workaround as no idea how to implement hash to use a HashMap
#[derive(Debug)]
struct LaserLineSight {
    line: Line,
    asteroid: Point,
}

impl PartialEq for LaserLineSight {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line
    }
}

impl LaserLineSight {
    fn sort_quadrant_1_3(data: &mut Vec<Self>) {
        data.sort_unstable_by(|a, b| {
            if a.line.dy == 0 {
                Ordering::Greater
            } else if a.line.dx == 0 {
                Ordering::Less
            } else if b.line.dy == 0 {
                Ordering::Less
            } else if b.line.dx == 0 {
                Ordering::Greater
            } else {
                let angle_a = (a.line.dy as f64 / a.line.dx as f64).atan();
                let angle_b = (b.line.dy as f64 / b.line.dx as f64).atan();

                if angle_a > angle_b {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        })
    }

    fn sort_quadrant_2_4(data: &mut Vec<Self>) {
        data.sort_unstable_by(|a, b| {
            if a.line.dy == 0 {
                Ordering::Less
            } else if a.line.dx == 0 {
                Ordering::Greater
            } else if b.line.dy == 0 {
                Ordering::Greater
            } else if b.line.dx == 0 {
                Ordering::Less
            } else {
                let angle_a = (a.line.dy as f64 / a.line.dx as f64).atan();
                let angle_b = (b.line.dy as f64 / b.line.dx as f64).atan();

                if angle_a > angle_b {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        })
    }
}

pub struct AsteroidsMap {
    asteroids: Vec<Point>,
}

impl AsteroidsMap {
    pub fn from(input: &str) -> Self {
        let mut asteroids: Vec<Point> = Vec::new();
        let mut x;
        for (y, line) in input.split('\n').enumerate() {
            x = 0;

            for item in line.trim().chars() {
                if item == '#' {
                    asteroids.push((x, y as u32));
                }

                x += 1;
            }
        }

        Self { asteroids }
    }

    // Simple version for part1. We could have used part2 with the base iteration
    pub fn calculate_best_base(&self) -> (usize, Option<Point>) {
        let mut base_position: Option<Point> = None;
        let mut max_asteroids = 0;

        for &base in self.asteroids.iter() {
            let asteroids_quadrant_1 = self.asteroids.iter().filter(|&&asteroid| {
                asteroid.0 >= base.0 && asteroid.1 <= base.1 && asteroid != base
            });
            let asteroids_quadrant_2 = self.asteroids.iter().filter(|&&asteroid| {
                asteroid.0 >= base.0 && asteroid.1 > base.1 && asteroid != base
            });
            let asteroids_quadrant_3 = self.asteroids.iter().filter(|&&asteroid| {
                asteroid.0 < base.0 && asteroid.1 > base.1 && asteroid != base
            });
            let asteroids_quadrant_4 = self.asteroids.iter().filter(|&&asteroid| {
                asteroid.0 < base.0 && asteroid.1 <= base.1 && asteroid != base
            });

            let monitored_asteroids_quadrant_1 =
                Self::get_number_monitored_asteroids(asteroids_quadrant_1, base);
            let monitored_asteroids_quadrant_2 =
                Self::get_number_monitored_asteroids(asteroids_quadrant_2, base);
            let monitored_asteroids_quadrant_3 =
                Self::get_number_monitored_asteroids(asteroids_quadrant_3, base);
            let monitored_asteroids_quadrant_4 =
                Self::get_number_monitored_asteroids(asteroids_quadrant_4, base);

            let total_monitored_asteroids = monitored_asteroids_quadrant_1
                + monitored_asteroids_quadrant_2
                + monitored_asteroids_quadrant_3
                + monitored_asteroids_quadrant_4;

            if total_monitored_asteroids > max_asteroids {
                max_asteroids = total_monitored_asteroids;
                base_position = Some(base);
            }
        }

        (max_asteroids, base_position)
    }

    // Simple version for part1. We could have used part2 one and just use the result's length
    fn get_number_monitored_asteroids<'a, T>(asteroids: T, base: Point) -> usize
    where
        T: Iterator<Item = &'a Point>,
    {
        let mut buffer = Vec::new();
        for &asteroid in asteroids {
            let line = Line::from(base, asteroid);

            if buffer.is_empty() || !buffer.contains(&line) {
                buffer.push(line);
            }
        }

        buffer.len()
    }

    pub fn calculate_vaporised_asteroids_queue(&self, base: Point) -> Vec<Point> {
        let mut vaporized_asteroids = Vec::new();

        // Vaporize everything except the base
        while vaporized_asteroids.len() != self.asteroids.len() - 1 {
            let asteroids_quadrant_1 = self.asteroids.iter().filter(|&&asteroid| {
                asteroid.0 >= base.0
                    && asteroid.1 <= base.1
                    && asteroid != base
                    && !vaporized_asteroids.contains(&asteroid)
            });
            let asteroids_quadrant_2 = self.asteroids.iter().filter(|&&asteroid| {
                asteroid.0 >= base.0
                    && asteroid.1 > base.1
                    && asteroid != base
                    && !vaporized_asteroids.contains(&asteroid)
            });
            let asteroids_quadrant_3 = self.asteroids.iter().filter(|&&asteroid| {
                asteroid.0 < base.0
                    && asteroid.1 > base.1
                    && asteroid != base
                    && !vaporized_asteroids.contains(&asteroid)
            });
            let asteroids_quadrant_4 = self.asteroids.iter().filter(|&&asteroid| {
                asteroid.0 < base.0
                    && asteroid.1 <= base.1
                    && asteroid != base
                    && !vaporized_asteroids.contains(&asteroid)
            });

            let mut vaporized_asteroids_quadrant_1 =
                Self::get_vaporised_asteroids(asteroids_quadrant_1, base);
            let mut vaporized_asteroids_quadrant_2 =
                Self::get_vaporised_asteroids(asteroids_quadrant_2, base);
            let mut vaporized_asteroids_quadrant_3 =
                Self::get_vaporised_asteroids(asteroids_quadrant_3, base);
            let mut vaporized_asteroids_quadrant_4 =
                Self::get_vaporised_asteroids(asteroids_quadrant_4, base);

            LaserLineSight::sort_quadrant_1_3(&mut vaporized_asteroids_quadrant_1);
            vaporized_asteroids_quadrant_1
                .iter()
                .for_each(|x| vaporized_asteroids.push(x.asteroid));

            LaserLineSight::sort_quadrant_2_4(&mut vaporized_asteroids_quadrant_2);
            vaporized_asteroids_quadrant_2
                .iter()
                .for_each(|x| vaporized_asteroids.push(x.asteroid));

            LaserLineSight::sort_quadrant_1_3(&mut vaporized_asteroids_quadrant_3);
            vaporized_asteroids_quadrant_3
                .iter()
                .for_each(|x| vaporized_asteroids.push(x.asteroid));

            LaserLineSight::sort_quadrant_2_4(&mut vaporized_asteroids_quadrant_4);
            vaporized_asteroids_quadrant_4
                .iter()
                .for_each(|x| vaporized_asteroids.push(x.asteroid));
        }

        vaporized_asteroids
    }

    fn get_vaporised_asteroids<'a, T>(asteroids: T, base: Point) -> Vec<LaserLineSight>
    where
        T: Iterator<Item = &'a Point>,
    {
        let mut buffer: Vec<LaserLineSight> = Vec::new();

        for &asteroid in asteroids {
            let line = Line::from(base, asteroid);
            let laser_line_sight = LaserLineSight { line, asteroid };

            let buffer_line_index = buffer.iter().position(|x| x == &laser_line_sight);

            if buffer.is_empty() || buffer_line_index.is_none() {
                buffer.push(laser_line_sight);
            } else {
                let line_point = &mut buffer[buffer_line_index.unwrap()].asteroid;

                let previous_asteroid_distance_base =
                    Self::asteroid_distance_to_base(*line_point, base);
                let new_asteroid_distance_base = Self::asteroid_distance_to_base(asteroid, base);

                if new_asteroid_distance_base < previous_asteroid_distance_base {
                    *line_point = asteroid;
                }
            }
        }

        buffer
    }

    fn asteroid_distance_to_base(asteroid: Point, base: Point) -> f64 {
        let distance_x = (asteroid.0 as i32 - base.0 as i32).abs();
        let distance_y = (asteroid.1 as i32 - base.1 as i32).abs();

        (distance_x.pow(2) as f64 + distance_y.pow(2) as f64).sqrt()
    }
}

type Point = (u32, u32);
