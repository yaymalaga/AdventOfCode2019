use std::collections::HashMap;

pub struct OrbitMap {
    data: HashMap<String, OrbitObject>,
}

struct OrbitObject {
    parent: Option<String>,
    children: Option<Vec<String>>,
}

struct OrbitStep {
    previous_orbit_id: Option<String>,
    current_orbit_id: String,
    steps_counter: usize,
}

impl OrbitMap {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add_orbit(&mut self, data: &str) {
        let orbit_data: Vec<&str> = data.split(")").collect();
        let parent = orbit_data[0].to_string();
        let child = orbit_data[1].to_string();

        if !self.data.contains_key(&child) {
            let object = OrbitObject::from(Some(parent.clone()), None);
            self.data.insert(child.to_string(), object);
        } else {
            let child_object = self.data.get_mut(&child).unwrap();

            if let Some(_) = &child_object.parent {
                panic!("Multiples parents are not allowed");
            } else {
                child_object.parent = Some(parent.clone());
            }
        }

        if !self.data.contains_key(&parent) {
            let object = OrbitObject::from(None, Some(vec![child]));
            self.data.insert(parent.to_string(), object);
        } else {
            let parent_object = self.data.get_mut(&parent).expect("Invalid parent");

            if let Some(x) = &mut parent_object.children {
                x.push(child);
            } else {
                parent_object.children = Some(vec![child]);
            }
        }
    }

    pub fn get_checksum(&self) -> usize {
        let mut counter = 0;
        let mut orbit_info: HashMap<String, (usize, usize)> = HashMap::new();

        'main: loop {
            for (id, object) in self.data.iter() {
                if !orbit_info.contains_key(id) {
                    match &object.parent {
                        None => {
                            orbit_info.insert(id.clone(), (0, 0));
                        }
                        Some(x) => {
                            let (direct, indirect) = if let Some(y) = orbit_info.get(x) {
                                (1, y.0 + y.1)
                            } else {
                                continue;
                            };

                            orbit_info.insert(id.clone(), (direct, indirect));
                            counter += direct + indirect;
                        }
                    }
                }
            }

            if orbit_info.len() == self.data.len() {
                break 'main;
            }
        }

        counter
    }

    pub fn get_minimum_orbit_transfers(&self, from: &str, to: &str) -> Option<usize> {
        let mut minimum_transfers_counter: Option<usize> = None;
        let mut queue: Vec<OrbitStep> = Vec::new();

        queue.push(OrbitStep::from(None, from.to_owned(), 0));

        while !queue.is_empty() {
            let orbit_step = queue.pop().expect("No orbit_step in the queue");
            let orbit_object = self.data.get(&orbit_step.current_orbit_id).unwrap();

            if let Some(children) = &orbit_object.children {
                for child in children {
                    // To function
                    let include_step = if let Some(previous) = &orbit_step.previous_orbit_id {
                        previous != child
                    } else {
                        true
                    };

                    if include_step {
                        queue.push(OrbitStep::from(
                            Some(orbit_step.current_orbit_id.clone()),
                            child.clone(),
                            orbit_step.steps_counter + 1,
                        ));
                    }
                }
            }

            if let Some(parent) = &orbit_object.parent {
                // To function
                let include_step = if let Some(previous) = &orbit_step.previous_orbit_id {
                    previous != parent
                } else {
                    true
                };

                if include_step {
                    queue.push(OrbitStep::from(
                        Some(orbit_step.current_orbit_id.clone()),
                        parent.clone(),
                        orbit_step.steps_counter + 1,
                    ));
                }
            }

            if orbit_step.current_orbit_id == to {
                let update_counter = if let Some(counter) = minimum_transfers_counter {
                    orbit_step.steps_counter > counter
                } else {
                    true
                };

                if update_counter {
                    minimum_transfers_counter = Some(orbit_step.steps_counter);
                }
            }
        }

        minimum_transfers_counter
    }
}

impl OrbitObject {
    fn from(parent: Option<String>, children: Option<Vec<String>>) -> Self {
        Self { parent, children }
    }
}

impl OrbitStep {
    fn from(
        previous_orbit_id: Option<String>,
        current_orbit_id: String,
        steps_counter: usize,
    ) -> Self {
        Self {
            previous_orbit_id,
            current_orbit_id,
            steps_counter,
        }
    }
}
