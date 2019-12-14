use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let mut prog = init_program(String::from("input"))?;

    prog[1] = 12;
    prog[2] = 2;

    let result = exec_prog(prog);
    println!("Pos 0: {}", result[0]);

    return Ok(());
}

fn init_program(file_name: String) -> Result<Vec<isize>, Box<dyn Error>> {
    let file = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let org_prog: Vec<&str> = contents.split(',').collect();

    let mut parsed_prog: Vec<isize> = Vec::new();

    for s in org_prog {
        match s.parse::<isize>() {
            Ok(i) => parsed_prog.push(i),
            Err(_) => {}
        }
    }

    return Ok(parsed_prog);
}

fn exec_prog(prog: Vec<isize>) -> Vec<isize> {
    let mut pos: usize = 0;
    let mut p = prog;
    while p[pos] != 99 {
        if p[pos] == 1 {
            let arg1_pos: usize = p[pos + 1] as usize;
            let arg2_pos: usize = p[pos + 2] as usize;
            let result_pos: usize = p[pos + 3] as usize;
            p[result_pos] = p[arg1_pos] + p[arg2_pos];
            pos += 4
        }
        if p[pos] == 2 {
            let arg1_pos: usize = p[pos + 1] as usize;
            let arg2_pos: usize = p[pos + 2] as usize;
            let result_pos: usize = p[pos + 3] as usize;
            p[result_pos] = p[arg1_pos] * p[arg2_pos];
            pos += 4
        }
    }
    return p;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_execution() {
        assert_eq!(exec_prog(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(exec_prog(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(
            exec_prog(vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            exec_prog(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
