use std::time::Duration;

#[derive(Debug, PartialEq, Eq)]
pub struct GameTime {
    pub hours: usize,
    pub minutes: usize,
    pub seconds: usize,
    pub milliseconds: usize,
}

impl GameTime {
    pub fn as_millisecs(&self) -> usize {
        let hours_ms = self.hours * 60 * 60 * 1_000;
        let minutes_ms = self.minutes * 60 * 1_000;
        let seconds_ms = self.seconds * 1_000;
        return hours_ms + minutes_ms + seconds_ms + self.milliseconds;
    }
}

#[test]
fn test_as_millisecs() {
    let subject = GameTime {
        hours: 2,
        minutes: 42,
        seconds: 14,
        milliseconds: 234,
    };
    assert_eq!(subject.as_millisecs(), 9_734_234);
}

pub trait DurationExt {
    fn from_game_time(game_time: &GameTime) -> Self;
}

impl DurationExt for Duration {
    fn from_game_time(game_time: &GameTime) -> Self {
        Self::from_millis(game_time.as_millisecs() as u64)
    }
}
