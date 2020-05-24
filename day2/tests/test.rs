use day2::Computer;

fn get_result(data: Vec<u32>) -> u32 {
    let mut computer = Computer::new();
    computer.load_program(data);
    computer.run();
    return computer.get_value(0);
}
#[test]
fn test1() {
    assert_eq!(
        get_result(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
        3500
    );
}

#[test]
fn test2() {
    assert_eq!(get_result(vec![1, 0, 0, 0, 99]), 2);
}

#[test]
fn test3() {
    assert_eq!(get_result(vec![2, 3, 0, 3, 99]), 2);
}

#[test]
fn test4() {
    assert_eq!(get_result(vec![2, 4, 4, 5, 99, 0]), 2);
}

#[test]
fn test5() {
    assert_eq!(get_result(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]), 30);
}
