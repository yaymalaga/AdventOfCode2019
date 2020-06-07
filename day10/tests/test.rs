use day10::AsteroidsMap;

#[test]
fn test1() {
    let input = "......#.#.
    #..#.#....
    ..#######.
    .#.#.###..
    .#..#.....
    ..#....#.#
    #..#....#.
    .##.#..###
    ##...#..#.
    .#....####";

    let asteroids_map = AsteroidsMap::from(input);

    assert_eq!( asteroids_map.calculate_best_base(), (33, Some((5,8))));
}

#[test]
fn test2() {
    let input = "#.#...#.#.
    .###....#.
    .#....#...
    ##.#.#.#.#
    ....#.#.#.
    .##..###.#
    ..#...##..
    ..##....##
    ......#...
    .####.###.";

    let asteroids_map = AsteroidsMap::from(input);

    assert_eq!( asteroids_map.calculate_best_base(), (35, Some((1,2))));
}

#[test]
fn test3() {
    let input = ".#..#..###
    ####.###.#
    ....###.#.
    ..###.##.#
    ##.##.#.#.
    ....###..#
    ..#.#..#.#
    #..#.#.###
    .##...##.#
    .....#.#..";

    let asteroids_map = AsteroidsMap::from(input);

    assert_eq!( asteroids_map.calculate_best_base(), (41, Some((6,3))));
}

#[test]
fn test4() {
    let input = ".#..##.###...#######
    ##.############..##.
    .#.######.########.#
    .###.#######.####.#.
    #####.##.#.##.###.##
    ..#####..#.#########
    ####################
    #.####....###.#.#.##
    ##.#################
    #####.##.###..####..
    ..######..##.#######
    ####.##.####...##..#
    .#####..#.######.###
    ##...#.##########...
    #.##########.#######
    .####.#.###.###.#.##
    ....##.##.###..#####
    .#.#.###########.###
    #.#.#.#####.####.###
    ###.##.####.##.#..##";

    let asteroids_map = AsteroidsMap::from(input);

    assert_eq!( asteroids_map.calculate_best_base(), (210, Some((11,13))));
}

#[test]
fn test5() {
    let input = ".#..##.###...#######
    ##.############..##.
    .#.######.########.#
    .###.#######.####.#.
    #####.##.#.##.###.##
    ..#####..#.#########
    ####################
    #.####....###.#.#.##
    ##.#################
    #####.##.###..####..
    ..######..##.#######
    ####.##.####...##..#
    .#####..#.######.###
    ##...#.##########...
    #.##########.#######
    .####.#.###.###.#.##
    ....##.##.###..#####
    .#.#.###########.###
    #.#.#.#####.####.###
    ###.##.####.##.#..##";

    let asteroids_map = AsteroidsMap::from(input);
    let base = (11,13);
    let vaporized_asteroids = asteroids_map.calculate_vaporised_asteroids_queue(base);

    assert_eq!( vaporized_asteroids[0], (11, 12));
    assert_eq!( vaporized_asteroids[1], (12, 1));
    assert_eq!( vaporized_asteroids[2], (12, 2));
    assert_eq!( vaporized_asteroids[9], (12, 8));
    assert_eq!( vaporized_asteroids[19], (16, 0));
    assert_eq!( vaporized_asteroids[49], (16, 9));
    assert_eq!( vaporized_asteroids[99], (10, 16));
    assert_eq!( vaporized_asteroids[198], (9, 6));
    assert_eq!( vaporized_asteroids[199], (8, 2));
    assert_eq!( vaporized_asteroids[200], (10, 9));
    assert_eq!( vaporized_asteroids[298], (11, 1));
}