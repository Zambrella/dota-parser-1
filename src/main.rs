mod game_time;
mod parse_game_time;
mod parse_hero_name;
mod parse_subject;

use ::std::env;
use ::std::fs;
use ::std::time::Duration;
use game_time::DurationExt;
use parse_game_time::parse_game_time;
use parse_subject::parse_subject;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let data = fs::read_to_string(&args[0]).expect("Failed to read file");
    parse_combat_log(&data);
}

fn parse_combat_log(data: &str) -> () {
    let lines = data.split("\n").into_iter();
    let mut kills: HashMap<String, u8> = HashMap::new();
    let mut deaths: HashMap<String, u8> = HashMap::new();
    for line in lines {
        // TODO: We really only want to split the string once. We could then pass the list to the
        // parsing functions

        let subject_and_killer = match parse_subject(line) {
            Some((subject, killer)) => (subject, killer),
            None => continue,
        };
        let subject = subject_and_killer.0;
        let killer = subject_and_killer.1;

        let just_time = match line.split_once(" ") {
            Some((a, _)) => a.trim(),
            None => continue,
        };
        let time = parse_game_time(just_time);
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
