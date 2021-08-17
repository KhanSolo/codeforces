use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let ch = line.trim().parse::<u32>().unwrap();
    if ch < 4 {
        println!("NO");
        return
    }

    let result = if ch % 2 == 0 { "YES" } else {"NO"};
    println!("{}", result);
}
