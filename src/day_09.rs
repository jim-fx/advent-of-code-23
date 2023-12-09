fn solve_line(line: Vec<i64>) -> i64 {
    let mut current_lines: Vec<Vec<i64>> = Vec::new();
    current_lines.push(line.clone());
    let mut line_index = 0;
    let length = line.len();
    while line_index < length - 1 {
        let current_line = current_lines.get(line_index).unwrap();
        let mut new_line = current_line.clone();
        new_line.fill(0);

        let mut line_total = 0;

        // start at the second index in the line
        // _ i 2 3 4 5 6 7 8 9 10
        for i in (line_index + 1)..length {
            let diff = current_line[i] - current_line[i - 1];
            line_total += diff;
            new_line[i] = diff;
        }

        if line_total == 0 {
            break;
        }

        current_lines.push(new_line);
        line_index += 1;
    }

    let mut end_num = *current_lines.last().unwrap().last().unwrap();
    for _i in 1..(line_index + 1) {
        let i = line_index - _i;
        end_num += current_lines[i][length - 1];
    }

    return end_num;
}

pub fn solve(input: String) -> (u64, u64) {
    let lines = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut total = 0;

    for line in lines {
        let res = solve_line(line);
        total += res;
    }
    println!("{}", total);
    return (0, 0);
}
