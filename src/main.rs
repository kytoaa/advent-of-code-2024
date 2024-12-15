mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;

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
    day_13::run(dir);
}
