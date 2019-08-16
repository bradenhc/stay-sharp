use std::io;

fn main() {
    print_table();
}

fn print_table() {
    // Capture the first parameter from stdin (i)
    let i: u32 = loop {
        let mut input_i = String::new();
        io::stdin()
            .read_line(&mut input_i)
            .expect("Failed to read 'i' line");
        match input_i.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        }
    };

    // For each input up to 'i' inputs...
    for _ in 0..i {
        // Capture the value of the second parameter from stdin (n)
        let n: u32 = loop {
            let mut input_n = String::new();
            io::stdin()
                .read_line(&mut input_n)
                .expect("Failed to read 'n' line");
            match input_n.trim().parse() {
                Ok(num) => break num,
                Err(_) => continue,
            }
        };

        // Print the table (multiplication from 1-10)
        for x in 1..11 {
            print!("{} ", n * x);
        }

        print!("\n");
    }
}
