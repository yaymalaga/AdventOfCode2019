use day4::{PasswordBox, PasswordType};

fn open_box(box_type: PasswordType, password: u32) -> bool {
    let password_box = PasswordBox::new(box_type, "240920-789857");
    password_box.check_password(password)
}
#[test]
fn simple_1() {
    assert_eq!(open_box(PasswordType::Simple, 111111), true);
}

#[test]
fn simple_2() {
    assert_eq!(open_box(PasswordType::Simple, 223450), false);
}

#[test]
fn simple_3() {
    assert_eq!(open_box(PasswordType::Simple, 123789), false);
}

#[test]
fn complex_1() {
    assert_eq!(open_box(PasswordType::Complex, 112233), true);
}

#[test]
fn complex_2() {
    assert_eq!(open_box(PasswordType::Complex, 123444), false);
}

#[test]
fn complex_3() {
    assert_eq!(open_box(PasswordType::Complex, 111122), true);
}