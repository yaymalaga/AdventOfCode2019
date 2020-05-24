pub struct Module {
    mass: u32
}

impl Module {
    pub fn from(mass: u32) -> Self {
        Self { mass }
    }

    pub fn initial_fuel(&self) -> u32 {
        Self::calculate_fuel(self.mass)
    }

    pub fn extra_fuel(&self) -> u32 {
        let mut tmp_fuel = self.initial_fuel();
        let mut extra_fuel: u32;
        let mut total_fuel: u32 = 0;

        loop {
            extra_fuel = Self::calculate_fuel(tmp_fuel);

            if extra_fuel == 0 {
                break;
            } 

            total_fuel += extra_fuel;
            tmp_fuel = extra_fuel;
        } 

        total_fuel
    }

    fn calculate_fuel(mass: u32) -> u32 {
        let division = (mass as f64 / 3 as f64).floor() as u32;
        if division > 2 {
            division - 2
        } else {
            0
        }
    }
}