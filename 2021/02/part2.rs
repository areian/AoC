use std::fs;

fn parse_command(command: &str) -> (isize, isize) {
    let cmd: Vec<&str> = command.split_whitespace().collect();
    match cmd[0] {
        "up" => (-1 * cmd[1].parse::<isize>().expect("NaN"), 0),
        "down" => (cmd[1].parse::<isize>().expect("NaN"), 0),
        "forward" => (0, cmd[1].parse::<isize>().expect("NaN")),
        _ => panic!("Unknown command"),
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Something went wrong reading the file");
    let (depth, pos, _) = input.lines().map(parse_command).fold(
        (0, 0, 0),
        |(depth, pos, aim), (aim_change, step)| {
            (depth + (aim * step), pos + step, aim + aim_change)
        },
    );
    println!("{}", depth * pos);
}
