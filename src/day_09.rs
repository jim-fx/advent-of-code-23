fn solve_line(line: Vec<i64>) -> i64 {
    let mut current_lines: Vec<Vec<i64>> = Vec::new();
    current_lines.push(line.clone());
    let mut line_index = 0;
    let length = line.len();
    while length > 0 {
        let current_line = current_lines.get(line_index).unwrap();
        let mut new_line = current_line.clone();
        new_line.fill(0);

        let mut finished = true;

        // start at the second index in the line
        // _ i 2 3 4 5 6 7 8 9 10
        for i in 1..(length - line_index) {
            let diff = current_line[i] - current_line[i - 1];
            if diff != 0 {
                finished = false;
            }
            new_line[i - 1] = diff;
        }

        if finished {
            break;
        }

        current_lines.push(new_line);
        line_index += 1;
    }

    let mut end_num = current_lines[line_index][0];
    for _i in 1..(line_index + 1) {
        let i = line_index - _i;
        end_num = current_lines[i][0] - end_num;
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
        println!("{}", res);
        total += res;
    }

    println!("{}", total);
    return (0, 0);
}
