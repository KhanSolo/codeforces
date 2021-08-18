use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let cnt = line.trim().parse::<u32>().unwrap();
    let mut slv = 0u32;
    for _i in 0..cnt {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let mut current = 0u32;
        for ch in line.chars() {
            if ch == '1' {
                current += 1;
            }
        }
        if current > 1 {
            slv +=1;
        }
    }
    println!("{}", slv);
}