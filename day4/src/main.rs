use day4::{PasswordBox, PasswordType};

fn main() {
    let password_box = PasswordBox::new(PasswordType::Simple, "240920-789857");
    println!("Result 1: {}", password_box.number_possible_passwords());

    let password_box = PasswordBox::new(PasswordType::Complex, "240920-789857");
    println!("Result 2: {}", password_box.number_possible_passwords());
}
