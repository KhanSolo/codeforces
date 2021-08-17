use std::collections::hash_set::HashSet;
use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let mut set = HashSet::new();
    for ch in line.chars() {
        if !set.contains(&ch) {
            set.insert(ch);
        }
    }

    let len = set.len();
    let remainder = len % 2;
    let result = if remainder == 0 {
        "CHAT WITH HER!"
    } else {
        "IGNORE HIM!"
    };
    println!("{}", result);
}
