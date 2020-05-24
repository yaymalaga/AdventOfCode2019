use day6::OrbitMap;

#[test]
fn test1() {
    let mut orbit_map = OrbitMap::new();

    for orbit in "COM)B,B)C,C)D,D)E,E)F,B)G,G)H,D)I,E)J,J)K,K)L".split(",") {
        orbit_map.add_orbit(&orbit);
    }

    assert_eq!(orbit_map.get_checksum(), 42);
}

#[test]
fn test2() {
    let mut orbit_map = OrbitMap::new();

    for orbit in "COM)B,B)C,C)D,D)E,E)F,B)G,G)H,D)I,E)J,J)K,K)L,K)YOU,I)SAN".split(",") {
        orbit_map.add_orbit(&orbit);
    }

    assert_eq!(orbit_map.get_minimum_orbit_transfers("YOU","SAN").unwrap() - 2, 4);
}