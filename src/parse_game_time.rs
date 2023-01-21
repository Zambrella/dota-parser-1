use regex::Regex;
use std::error::Error;
use std::fmt;

use crate::game_time::GameTime;

/// Expected format: "[HH:MM:SS:ms]"
pub fn parse_game_time(time: &str) -> Result<GameTime, ParseError> {
    let re = Regex::new(r"\[(.+)\]");
    let regex = match re {
        Ok(regex) => regex,
        Err(_) => {
            return Err(ParseError);
        }
    };
    let capture = regex.captures(time);
    let unformatted_time = match capture {
        Some(cap) => match cap.get(1) {
            Some(time) => time.as_str(),
            None => return Err(ParseError),
        },
        None => {
            return Err(ParseError);
        }
    };
    let re = Regex::new(r"(\d\d):(\d\d):(\d\d)\.(\d\d\d)");
    let regex = match re {
        Ok(regex) => regex,
        Err(_) => return Err(ParseError),
    };
    let capture = regex.captures(unformatted_time);
    match capture {
        Some(capture) => {
            let hours = capture.get(1).unwrap().as_str().parse().unwrap();
            let minutes = capture.get(2).unwrap().as_str().parse().unwrap();
            let seconds = capture.get(3).unwrap().as_str().parse().unwrap();
            let milliseconds = capture.get(4).unwrap().as_str().parse().unwrap();
            return Ok(GameTime {
                hours,
                minutes,
                seconds,
                milliseconds,
            });
        }
        None => return Err(ParseError),
    }
}

#[derive(Debug)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for ParseError {}

#[test]
fn test_parse_game_time() {
    let input = "[00:16:24.781]";
    let output = parse_game_time(input).unwrap();
    assert_eq!(
        output,
        GameTime {
            hours: 0,
            minutes: 16,
            seconds: 24,
            milliseconds: 781
        }
    );
}

#[test]
#[should_panic]
fn test_parse_game_time_with_invalid_input() {
    let input = "abcdg";
    let _output = parse_game_time(input).unwrap();
}
