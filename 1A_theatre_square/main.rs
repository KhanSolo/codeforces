use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let split: Vec<&str> = line.trim().split(" ").collect();

    let m : u64 = split[0].parse().unwrap();
    let n : u64 = split[1].parse().unwrap();
    let a : u64 = split[2].parse().unwrap();
    //dbg!(m, n, a);

    let rows = count(m, a);
    let cols = count(n, a);
    let total = rows * cols;

    println!("{}", total);
}

fn count(k : u64, a : u64) -> u64 {
    k / a + if k % a > 0 { 1 } else { 0 }
}