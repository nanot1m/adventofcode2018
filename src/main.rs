mod aoc;
mod day1;
mod day2;

#[tokio::main]
async fn main() {
    day2::solve().await;
}