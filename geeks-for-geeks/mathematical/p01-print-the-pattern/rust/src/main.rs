fn main() {
    print_the_pattern(10);
}

fn print_the_pattern(n: i32) {

    let mut s = String::new();

    for i in (1..n + 1).rev() {
        for j in (1..n + 1).rev() {
            for _ in 0..i {
                s.push_str(j.to_string().as_str());
            }
        }
        s.push_str("\n");
    }

    print!("{}", s);
}
