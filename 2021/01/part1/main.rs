use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("../input").expect("File not found");
    let mut reader = BufReader::new(f);
    let mut line = String::new();

    reader.read_line(&mut line).expect("Could not read line");

    print!("{}", line);
    let mut last = line.trim_end().parse::<i32>().expect("Not a number");
    line.clear();
    let mut incr = 0;

    while reader.read_line(&mut line).expect("Could not read line") > 0 {
      print!("{}", line);
      let i = line.trim_end().parse::<i32>().expect("Not a number");
      if i > last {
        incr = incr + 1;
      }
      last = i;
      line.clear();
    }

    println!("Increases: {}", incr);
}
