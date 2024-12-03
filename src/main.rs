mod day1;
mod day2;
mod day3;

fn main() {
    let dir = String::from(match std::env::var("AOC_2024_INPUTS") {
        Ok(d) => {
            assert!(std::fs::metadata(&d).is_ok());
            d
        }
        Err(_) => {
            println!("AOC_2024_INPUTS environmental variable does not exist");
            return;
        }
    });
    day3::run(dir);
}
