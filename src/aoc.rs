use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;

async fn fetch_input(day: u8) -> Result<String, Box<dyn Error>> {
    dotenv::dotenv().ok();

    let url = format!("https://adventofcode.com/2018/day/{}/input", day);
    let cookie = env::var("AOC_SESSION")?;
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("Cookie", format!("session={}", cookie))
        .send()
        .await?
        .text()
        .await?;
    let path = format!("input/day{}.txt", day);
    let mut file = File::create(path)?;
    file.write_all(resp.as_bytes())?;
    return Ok(resp);
}

async fn read_input(day: u8) -> Result<String, Box<dyn Error>> {
    let path = format!("input/day{}.txt", day);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}

pub async fn read_input_or_fetch(day: u8) -> Result<String, Box<dyn Error>> {
    let path = format!("input/day{}.txt", day);
    let file = File::open(path);
    match file {
        Ok(_) => read_input(day).await,
        Err(_) => fetch_input(day).await,
    }
}
