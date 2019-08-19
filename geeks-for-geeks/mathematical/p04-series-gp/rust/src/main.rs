fn main() {
    match get_floored_nth_of_geometric_progression(84.0, 87.0, 3.0) {
        Ok(result) => println!("{}", result),
        Err(err) => println!("{}", err),
    };
}

const MIN: f32 = -100.0;
const MAX: f32 = 100.0;
const N_MIN: f32 = 1.0;
const N_MAX: f32 = 5.0;

fn get_floored_nth_of_geometric_progression(a: f32, b: f32, n: f32) -> Result<i32, String> {
    if a < MIN || a > MAX || b < MIN || b > MAX || n < N_MIN || n > N_MAX {
        Err(String::from("invalid arguments"))
    } else {
        Ok((a * (b / a).powf(n - 1.0)) as i32)
    }
}
