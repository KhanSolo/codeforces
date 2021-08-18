use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let cnt = line.trim().parse::<u32>().unwrap();
    for _i in 0..cnt {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();
        let len = line.len();

        if len > 10 {
            println!("{}{}{}", line.chars().nth(0).unwrap(), len-2, line.chars().nth(len-1).unwrap())
        }
        else {
            println!("{}", line);
        }
    }
}