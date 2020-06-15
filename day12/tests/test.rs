use day12::{Position, Space};

fn simulate(data: &str, n_steps: usize) -> i64 {
    let mut space = Space::new();
    
    for line in data.split('\n') {
        let moon_position = Position::from(line);
        space.add_moon(moon_position);
    }

    space.run_steps(n_steps);

    space.get_total_energy()
}

#[test]
fn test_1() {
    let data = "<x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>";

    assert_eq!(simulate(data, 10), 179)
}

#[test]
fn test_2() {
    let data = "<x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>";

    assert_eq!(simulate(data, 100), 1940)
}