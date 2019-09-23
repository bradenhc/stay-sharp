use std::env;
use std::fs;
use std::io;

use std::io::BufRead;
use std::io::Write;

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{:?}", e);
            1
        }
    });
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(String::from("usage: cargo run <testcase_file>"));
    }

    let test_file = &args[1];

    let f = match fs::File::open(test_file) {
        Ok(file) => file,
        Err(e) => return Err(format!("Error reading file: {:?}", e)),
    };

    let mut reader = io::BufReader::new(&f);
    let mut writer = io::BufWriter::new(io::stdout());

    // Get the number of test cases
    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to read first line: {:?}", e)),
    };

    let mut t: i32 = match line.trim().parse() {
        Ok(n) => n,
        Err(e) => return Err(format!("Failed to parse number of test cases: {:?}", e)),
    };

    // Read the test cases
    for line in reader.lines() {

        if t == 0 {
            break;
        }

        let l = match line {
            Ok(r) => r,
            Err(e) => return Err(format!("Error iterating over lines: {:?}", e)),
        };

        let n: i32 = match l.trim().parse() {
            Ok(n) => n,
            Err(e) => return Err(format!("Error parsing value for 'n': {:?}", e)),
        };

        let result = if is_armstrong_number(n) { "Yes" } else { "No" };
        match write!(&mut writer, "{}\n", result) {
            Ok(_) => (),
            Err(e) => return Err(format!("Error writing result to stdout: {:?}", e)),
        };

        t -= 1;
    }

    Ok(())
}

fn is_armstrong_number(n: i32) -> bool {
    let mut sum: i32 = 0;
    let mut x = n;
    while x > 0 {
        sum += (x % 10).pow(3);
        x /= 10;
    }
    sum == n
}