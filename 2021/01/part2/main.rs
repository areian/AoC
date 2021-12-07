use std::fs;

fn main() {
    let input = fs::read_to_string("../input").expect("Something went wrong reading the file");
    let inputs: Vec<i32> = input.lines().map(|x| x.parse::<i32>().expect("NaN")).collect();

    let mut w = inputs[0] + inputs[1] + inputs[2];
    let mut incr = 0;

    for i in 1..inputs.len()-2 {
        let i = inputs[i] + inputs[i+1] + inputs[i+2];
        if i > w {
            incr = incr + 1;
        }
        w = i;
    }

    println!("{}", incr);
}
