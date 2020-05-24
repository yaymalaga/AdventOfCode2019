use day3::Wire;

fn create_wire(coordenates: &str) -> Wire {
    let mut wire = Wire::new();
    for coordenate in coordenates.split(',') {
        wire.add_coordenate(coordenate);
    }
    wire
}

#[test]
fn test1() {
    let wire1 = create_wire("R8,U5,L5,D3");
    let wire2 = create_wire("U7,R6,D4,L4");
    assert_eq!(
        Wire::nearest_intersection_distance(&wire1, &wire2),
        Some(6)
    );
}

#[test]
fn test2() {
    let wire1 = create_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let wire2 = create_wire("U62,R66,U55,R34,D71,R55,D58,R83");
    assert_eq!(
        Wire::nearest_intersection_distance(&wire1, &wire2),
        Some(159)
    );
}

#[test]
fn test3() {
    let wire1 = create_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let wire2 = create_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    assert_eq!(
        Wire::nearest_intersection_distance(&wire1, &wire2),
        Some(135)
    );
}

#[test]
fn test4() {
    let wire1 = create_wire("R8,U5,L5,D3");
    let wire2 = create_wire("U7,R6,D4,L4");
    assert_eq!(
        Wire::nearest_intersection_steps(&wire1, &wire2),
        Some(30)
    );
}

#[test]
fn test5() {
    let wire1 = create_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let wire2 = create_wire("U62,R66,U55,R34,D71,R55,D58,R83");
    assert_eq!(
        Wire::nearest_intersection_steps(&wire1, &wire2),
        Some(610)
    );
}

#[test]
fn test6() {
    let wire1 = create_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let wire2 = create_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    assert_eq!(
        Wire::nearest_intersection_steps(&wire1, &wire2),
        Some(410)
    );
}