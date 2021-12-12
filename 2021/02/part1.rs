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
    let (depth, pos) = input
        .lines()
        .map(parse_command)
        .reduce(|(depth, pos), (depth_change, step)| (depth + depth_change, pos + step))
        .unwrap();
    println!("{}", depth * pos);
}
