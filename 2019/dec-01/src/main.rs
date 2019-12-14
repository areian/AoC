use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);

    let mut total_fuel_required = 0;
    for line in buf_reader.lines() {
        let mass: isize = line?.parse().unwrap();
        total_fuel_required += fuel_required(mass);
    }

    println!("Total fuel required: {}", total_fuel_required);

    return Ok(());
}

fn fuel_required(mass: isize) -> isize {
    let mut fuel = mass / 3 - 2;

    if fuel / 3 - 2 > 0 {
        fuel += fuel_required(fuel);
    }
    return fuel;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_required() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 966);
        assert_eq!(fuel_required(100756), 50346);
    }
}
