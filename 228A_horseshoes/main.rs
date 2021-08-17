use std::collections::hash_set::HashSet;
use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let split = line.trim().split(" ");
    let mut set : HashSet<u32> = HashSet::new();

    for substr in split {
        let ch = substr.parse::<u32>().unwrap();
        if !set.contains(&ch) {
            set.insert(ch);
        }
    }

    let len = set.len();
    let result = 4 - len;
    println!("{}", result);
}
