pub struct PasswordBox {
    password_type: PasswordType,
    lower_limit: u32,
    upper_limit: u32,
}

pub enum PasswordType {
    Simple,
    Complex,
}

impl PasswordBox {
    pub fn new(password_type: PasswordType, limits: &str) -> Self {
        let lower_limit: u32 = limits
            .split('-')
            .nth(0)
            .expect("Invalid lower input")
            .parse()
            .expect("Invalid lower input number");
        let upper_limit: u32 = limits
            .split('-')
            .nth(1)
            .expect("Invalid upper input")
            .parse()
            .expect("Invalid upper input number");

        Self {
            password_type,
            lower_limit,
            upper_limit,
        }
    }

    pub fn number_possible_passwords(&self) -> usize {
        let mut possibilities: usize = 0;
        for possibility in self.lower_limit..=self.upper_limit {
            if self.check_password(possibility) {
                possibilities += 1;
            }
        }

        possibilities
    }

    pub fn check_password(&self, password: u32) -> bool {
        if !self.check_adjacents(password) {
            return false;
        }

        if !Self::check_no_decrease(password) {
            return false;
        }

        true
    }

    fn check_adjacents(&self, password: u32) -> bool {
        if let PasswordType::Simple = self.password_type {
            Self::check_adjacents_simple(password)
        } else {
            Self::check_adjacents_complex(password)
        }
    }

    fn check_adjacents_simple(password: u32) -> bool {
        let adjacents = password
            .to_string()
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .fold(
                0,
                |result, x| if x[0] == x[1] { result + 1 } else { result },
            );

        if adjacents > 0 {
            true
        } else {
            false
        }
    }

    fn check_adjacents_complex(password: u32) -> bool {
        let mut adjacents = 0;

        let mut last_adjacent_digit = None;
        let mut digit_counter = 0;
        let password_string = password.to_string();
        password_string
            .chars()
            .enumerate()
            .for_each(|(i, x)| match last_adjacent_digit {
                None => {
                    last_adjacent_digit = Some(x);
                    digit_counter = 1;
                }
                Some(y) => {
                    if x == y {
                        digit_counter += 1;
                        if i == password_string.len() - 1 && digit_counter == 2 {
                            adjacents += 1;
                        }
                    } else {
                        last_adjacent_digit = Some(x);
                        if digit_counter == 2 {
                            adjacents += 1;
                        }
                        digit_counter = 1;
                    }
                }
            });

        if adjacents > 0 {
            true
        } else {
            false
        }
    }

    fn check_no_decrease(password: u32) -> bool {
        let mut tmp = None;

        for digit in password.to_string().chars() {
            match tmp {
                None => tmp = Some(digit),
                Some(x) => {
                    if digit.to_digit(10).expect("Invalid digit")
                        < x.to_digit(10).expect("Invalid digit")
                    {
                        return false;
                    } else {
                        tmp = Some(digit);
                    }
                }
            }
        }

        true
    }
}