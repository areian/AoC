use std::fs;

//                                  depth, position, aim
fn parse_command(command: &str) -> (isize, isize, isize) {
    let cmd: Vec<&str> = command.split_whitespace().collect();
    match cmd[0] {
        "up" => (0, 0, -1 * cmd[1].parse::<isize>().expect("NaN")),
        "down" => (0, 0, cmd[1].parse::<isize>().expect("NaN")),
        "forward" => (0, cmd[1].parse::<isize>().expect("NaN"), 0),
        _ => panic!("Unknown command"),
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Something went wrong reading the file");
    let inputs = input
        .lines()
        .map(parse_command)
        .reduce(|acc, x| {
            if x.1 == 0 {
                (acc.0, acc.1, acc.2 + x.2)
            } else {
                (acc.0 + (acc.2 * x.1), acc.1 + x.1, acc.2)
            }
        })
        .unwrap();
    println!("{}", inputs.0 * inputs.1);
}
