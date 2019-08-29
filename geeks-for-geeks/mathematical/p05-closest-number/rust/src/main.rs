use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::io::BufRead;

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

        let inputs = l.split(' ').collect::<Vec<&str>>();

        let n: i32 = match inputs[0].trim().parse() {
            Ok(n) => n,
            Err(e) => return Err(format!("Error parsing value for 'n': {:?}", e)),
        };

        let m: i32 = match inputs[1].trim().parse() {
            Ok(m) => m,
            Err(e) => return Err(format!("Error parsing value for 'm': {:?}", e)),
        };

        let result = get_closest_number(n, m);
        match write!(&mut writer, "{}\n", result) {
            Ok(_) => (),
            Err(e) => return Err(format!("Error writing result to stdout: {:?}", e))
        };

        t -= 1;
    }

    Ok(())
}

fn get_closest_number(n: i32, m: i32) -> i32 {
    // This may look like it doesn't do anything, but we are dealing with integers, so `(n / m)`
    // is essentially a dividing `floor` operation, finding the largest number of times `m` fits
    // inside `n`
    if (n % m).abs() == m.abs() / 2 {
        let floored = n / m;
        let correction = if floored < 0 {
            -1
        } else {
            1
        };
        (floored + correction) * m
    } else {
        (n / m) * m
    }
}
