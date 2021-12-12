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
    let inputs = input
        .lines()
        .map(parse_command)
        //     depth, position, aim
        .fold((0, 0, 0), |acc, x| (acc.0 + (acc.2 * x.1), acc.1 + x.1, acc.2 + x.0));
    println!("{}", inputs.0 * inputs.1); // 1963088820
}
