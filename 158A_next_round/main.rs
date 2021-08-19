use std::io;

fn main() {

    let k : usize;
    {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let split: Vec<&str> = line.trim().split(" ").collect();

        k = split[1].parse().unwrap();
    }

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let split: Vec<&str> = line.trim().split(" ").collect();

    let v : u32 = split[k-1].parse().unwrap();
    let mut count : u32 = 0;
    for e in split{
        let p : u32 = e.parse().unwrap();
        if p > 0 && p >= v {
            count += 1;
        }
    }
    println!("{}", count);
}