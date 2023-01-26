mod game_time;
mod kills;
mod parse_game_time;
mod parse_hero_name;
mod pudge;

use ::std::fs;
use ::std::time::Duration;
use clap::Parser;
use game_time::DurationExt;
use kills::parse_subject;
use parse_game_time::parse_game_time;
use std::collections::HashMap;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    path: std::path::PathBuf,

    // allowed values: kills, pudge
    #[arg(short, long)]
    action: String,
}

fn main() {
    let args = Args::parse();
    let data = fs::read_to_string(&args.path).expect("Failed to read file");
    if &args.action.to_lowercase() == "kills" {
        parse_kills_and_deaths(&data);
    }

    if &args.action.to_lowercase() == "pudge" {
        parse_pudge_hooks(&data);
    }
}

fn parse_pudge_hooks(data: &str) -> () {
    let result = pudge::get_hook_percentage::get_hook_percentage(data);
    println!("Percentage of hooks hit: {:.2}%", result * 100.);
}

fn parse_kills_and_deaths(data: &str) -> () {
    let lines = data.split("\n").into_iter();
    let mut kills: HashMap<String, u8> = HashMap::new();
    let mut deaths: HashMap<String, u8> = HashMap::new();
    for line in lines {
        // TODO: We really only want to split the string once. We could then pass the list to the
        // parsing functions

        let subject_and_killer = match parse_subject::parse_subject(line) {
            Some((subject, killer)) => (subject, killer),
            None => continue,
        };
        let subject = subject_and_killer.0;
        let killer = subject_and_killer.1;

        let time = parse_game_time(line);
        let duration = match time {
            Ok(time) => Duration::from_game_time(&time),
            Err(_) => continue,
        };

        println!("{} killed {} at {:?}", killer, subject, duration);

        match kills.get(&killer) {
            Some(count) => {
                kills.insert(killer, count + 1);
            }
            None => {
                kills.insert(killer, 1);
            }
        }

        match deaths.get(&subject) {
            Some(count) => {
                deaths.insert(subject, count + 1);
            }
            None => {
                deaths.insert(subject, 1);
            }
        }
    }

    println!("Kill Result: {:?}", kills);
    println!("Death Result: {:?}", deaths);
}
