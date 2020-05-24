use day1::Module;

fn get_module_fuel(mass: u32) -> u32 {
    let module = Module::from(mass);
    module.initial_fuel()
}

fn get_total_fuel(mass: u32) -> u32 {
    let module = Module::from(mass);
    module.initial_fuel() + module.extra_fuel()
}

#[test]
fn test1() {
    assert_eq!(get_module_fuel(12), 2);
}

#[test]
fn test2() {
    assert_eq!(get_module_fuel(14), 2);
}

#[test]
fn test3() {
    assert_eq!(get_module_fuel(1969), 654);
}

#[test]
fn test4() {
    assert_eq!(get_module_fuel(100756), 33583);
}

#[test]
fn test5() {
    assert_eq!(get_total_fuel(14), 2);
}

#[test]
fn test6() {
    assert_eq!(get_total_fuel(1969), 966);
}

#[test]
fn test7() {
    assert_eq!(get_total_fuel(100756), 50346);
}