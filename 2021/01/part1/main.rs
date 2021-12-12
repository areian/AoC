use std::fs;

fn main() {
    let input = fs::read_to_string("../input").expect("Something went wrong reading the file");
    let (incr, _) = input
        .lines()
        .map(|line| line.parse::<i32>().expect("Not a number"))
        .fold((0, 0), |(incr, last), i| {
            if i > last {
                (incr + 1, i)
            } else {
                (incr, i)
            }
        });
    println!("{}", incr - 1); // don't count increase on first line
}
