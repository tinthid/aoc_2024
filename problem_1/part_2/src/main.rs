use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let mut list_1: Vec<i32> = Vec::new();
    let mut counter: HashMap<i32, i32> = HashMap::new();

    for line in io::stdin().lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();
        list_1.push(numbers[0].parse::<i32>().unwrap());
        *counter
            .entry(numbers[1].parse::<i32>().unwrap())
            .or_insert(0) += 1;
    }

    let result: i32 = list_1
        .iter()
        .map(|x| counter.get(&x).unwrap_or(&0) * x)
        .sum();

    println!("{}", result);

    Ok(())
}
