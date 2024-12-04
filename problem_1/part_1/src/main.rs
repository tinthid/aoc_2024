use std::io;

fn main() -> io::Result<()> {
    let mut list_1: Vec<i32> = Vec::new();
    let mut list_2: Vec<i32> = Vec::new();

    for line in io::stdin().lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();
        list_1.push(numbers[0].parse::<i32>().unwrap());
        list_2.push(numbers[1].parse::<i32>().unwrap());
    }

    list_1.sort();
    list_2.sort();

    let result: i32 = list_1
        .iter()
        .zip(list_2.iter())
        .map(|(x, y)| (x - y).abs())
        .sum();

    println!("{}", result);

    Ok(())
}
