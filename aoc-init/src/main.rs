use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "aoc-init",
    about = "automatically initializes Rust projects for Advent Of Code"
)]
struct Cli {
    #[structopt(short, long)]
    year: i32,
    #[structopt(short, long)]
    day: i32,
    #[structopt(env = "AOC_SESSION")]
    session: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::from_args();
    let puzzle_input = get_puzzle_input(&cli).await?;
    println!("{}", puzzle_input);
    Ok(())
}

async fn create_project(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let dir_name = format!("cargo new --bin day-{}", cli.day);
    Command::new("sh").arg("-c").arg(&dir_name).output()?;
    std::fs::File::create(PathBuf::from(&dir_name).join("src").join("input.txt"))?
        .write_all(get_puzzle_input(cli).await?.as_bytes());
    Ok(())
}

async fn get_puzzle_input(cli: &Cli) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        cli.year, cli.day
    );
    let client = reqwest::Client::new();
    let resp: String = client
        .get(url)
        .header(reqwest::header::COOKIE, format!("session={};", cli.session))
        .send()
        .await?
        .text()
        .await?;
    Ok(resp)
}
