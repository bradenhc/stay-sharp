use std::io;

fn main() {
    
    let t: i32 = read_i32_from_stdin();

    for _ in 0..t {
        let (a, b) = read_a_b_from_stdin();
        let n: i32 = read_i32_from_stdin();
        let result = series_ap(a, b, n);

        println!("{}", result);
    }

}

fn read_i32_from_stdin() -> i32 {
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read number");
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue
        }
    }
}

fn read_a_b_from_stdin() -> (i32, i32) {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read a and b from stdin");
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    let a = parts[0].parse().expect("Failed to parse a");
    let b = parts[1].parse().expect("Failed to parse b");

    (a, b)
}

fn series_ap(mut a: i32, b: i32, mut n: i32) -> i32 {
    // Determine the difference
    let d = b - a;

    // Starting from 'a', apply 'd' 'n' times
    while n - 1 > 0  {
        a += d;
        n -= 1;
    }

    return a;
}