fn solve_race(input: (usize, usize)) -> usize {
    let time = input.0 as f64;
    let distance = input.1 as f64;

    // x1 = (race_duration+sqrt(race_duration^2 - 4max_last_win))/2;
    // x2 = (race_duration-sqrt(race_duration^2 - 4max_last_win))/2;
    let x1 = (time - (time.powf(2.0) - 4.0 * distance).sqrt()) / 2.0 + 0.00001;
    let x2 = (time + (time.powf(2.0) - 4.0 * distance).sqrt()) / 2.0 - 0.00001;

    let i1 = x1.ceil() as usize;
    let i2 = x2.floor() as usize;
    let res = (i2 - i1 + 1) as usize;
    println!("x1: {}, x2: {}, i1: {}, i2: {} -> {}", x1, x2, i1, i2, res);

    return res;
}

pub fn solve(input: String) -> (u32, u32) {
    let lines = input.lines().collect::<Vec<&str>>();

    let first_line = lines
        .first()
        .expect("")
        .strip_prefix("Time: ")
        .unwrap()
        .split_whitespace();
    let last_line = lines
        .last()
        .expect("")
        .strip_prefix("Distance: ")
        .unwrap()
        .split_whitespace();

    // parsing
    let races1 = first_line
        .clone()
        .map(|s| s.parse::<usize>().unwrap())
        .zip(
            last_line
                .clone()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        )
        .collect::<Vec<(usize, usize)>>();

    let races2 = vec![
        first_line
            .collect::<Vec<&str>>()
            .join("")
            .parse::<usize>()
            .unwrap(),
        last_line
            .collect::<Vec<&str>>()
            .join("")
            .parse::<usize>()
            .unwrap(),
    ];

    let mut part1 = 0;

    for race in races1 {
        if part1 == 0 {
            part1 = solve_race(race)
        } else {
            part1 = part1 * solve_race(race)
        }
    }

    println!("{:?}", races2);
    let part2 = solve_race((races2[0], races2[1]));

    println!("part1: {}", part1);
    println!("part2: {}", part2);

    return (0, 0);
}
