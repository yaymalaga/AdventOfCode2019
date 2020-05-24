pub struct Wire {
    path: Vec<WireStep>,
}
#[derive(Copy, Clone)]
struct WireStep {
    x: i32,
    y: i32,
    steps: StepCounter,
}

#[derive(Copy, Clone)]
struct StepCounter {
    x: usize,
    y: usize,
}

enum Direction {
    Left,
    Up,
    Right,
    Down,
}

enum Logic {
    Steps,
    Distance,
}

impl Wire {
    pub fn new() -> Self {
        Self { path: Vec::new() }
    }

    pub fn add_coordenate(&mut self, data: &str) {
        let direction =
            Direction::from(data.chars().nth(0).expect("Coordinate direction not found"));
        let steps: usize = data
            .get(1..)
            .expect("Invalid steps data")
            .parse()
            .expect("Invalid steps number");

        self.path.reserve(steps);

        for _ in 1..=steps {
            self.add_step(&direction);
        }
    }

    fn add_step(&mut self, direction: &Direction) {
        let mut next_step = self.path.last().cloned().unwrap_or(WireStep::new());

        match direction {
            Direction::Left => {
                next_step.x -= 1;
                next_step.steps.x += 1;
            }
            Direction::Up => {
                next_step.y += 1;
                next_step.steps.y += 1;
            }
            Direction::Right => {
                next_step.x += 1;
                next_step.steps.x += 1;
            }
            Direction::Down => {
                next_step.y -= 1;
                next_step.steps.y += 1;
            }
        };

        self.path.push(next_step);
    }

    pub fn nearest_intersection_steps(wire_a: &Wire, wire_b: &Wire) -> Option<usize> {
        Self::nearest_intersection_by(Logic::Steps, wire_a, wire_b)
    }

    pub fn nearest_intersection_distance(wire_a: &Wire, wire_b: &Wire) -> Option<usize> {
        Self::nearest_intersection_by(Logic::Distance, wire_a, wire_b)
    }

    fn nearest_intersection_by(logic: Logic, wire_a: &Wire, wire_b: &Wire) -> Option<usize> {
        let intersections = Self::get_intersections(wire_a, wire_b)?;

        let mut result = None;

        for intersection in intersections {
            let tmp = match logic {
                Logic::Distance => (intersection.0.x.abs() + intersection.0.y.abs()) as usize,
                Logic::Steps => {
                    intersection.0.steps.x
                        + intersection.1.steps.x
                        + intersection.0.steps.y
                        + intersection.1.steps.y
                }
            };

            match result {
                None => result = Some(tmp),
                Some(x) if tmp < x => result = Some(tmp),
                _ => (),
            }
        }

        result
    }

    fn get_intersections<'a, 'b>(
        wire_a: &'a Wire,
        wire_b: &'b Wire,
    ) -> Option<Vec<(&'a WireStep, &'b WireStep)>> {
        let mut intersections = Vec::new();

        for step_wire_a in wire_a.path.iter() {
            for step_wire_b in wire_b.path.iter() {
                if step_wire_a.equals(&step_wire_b) {
                    intersections.push((step_wire_a, step_wire_b));
                }
            }
        }

        if intersections.is_empty() {
            None
        } else {
            Some(intersections)
        }
    }
}

impl WireStep {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            steps: StepCounter { x: 0, y: 0 },
        }
    }

    fn equals(self, wire: &Self) -> bool {
        self.x == wire.x && self.y == wire.y
    }
}

impl Direction {
    fn from(data: char) -> Self {
        match data {
            'L' => Direction::Left,
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            _ => panic!("Invalid direction"),
        }
    }
}
