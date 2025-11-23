use std::cell::LazyCell;
use std::fmt::Display;
use std::str::FromStr;
use std::time::Duration;
use std::u64;

use regex::Regex;

use crate::error::Error;

const MILLISECONDS_PER_SECOND: u64 = 1000;
const MILLISECONDS_PER_MINUTE: u64 = 60_000;
const MILLISECONDS_PER_HOUR: u64 = 3_600_000;
const SECONDS_PER_MINUTE: u64 = 60;
const MINUTES_PER_HOUR: u64 = 60;

const TIMESTAMP_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new("^(?<hours>\\d{2}):(?<minutes>\\d{2}):(?<seconds>\\d{2}),(?<milliseconds>\\d{3})$")
        .unwrap()
});

const TIMESTAMP_RANGE_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new("^(?<start>\\d{2}:\\d{2}:\\d{2},\\d{3}) --> (?<end>\\d{2}:\\d{2}:\\d{2},\\d{3})$")
        .unwrap()
});

struct Components {
    hours: u64,
    minutes: u64,
    seconds: u64,
    milliseconds: u64,
}

impl Display for Components {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:0>2}:{:0>2}:{:0>2},{:0>3}",
            self.hours, self.minutes, self.seconds, self.milliseconds
        )
    }
}

impl From<&Duration> for Components {
    fn from(value: &Duration) -> Self {
        let milliseconds = value.as_millis() as u64 % MILLISECONDS_PER_SECOND;
        let seconds = (value.as_millis() as u64 / MILLISECONDS_PER_SECOND) % SECONDS_PER_MINUTE;
        let minutes = (value.as_millis() as u64 / MILLISECONDS_PER_MINUTE) % MINUTES_PER_HOUR;
        let hours = value.as_millis() as u64 / MILLISECONDS_PER_HOUR;

        Self {
            hours,
            minutes,
            seconds,
            milliseconds,
        }
    }
}

pub struct Timestamp {
    inner: Duration,
}

impl Timestamp {
    fn new(duration: Duration) -> Self {
        Self { inner: duration }
    }

    fn components(&self) -> Components {
        Components::from(&self.inner)
    }

    pub fn as_string(&self) -> String {
        format!("{self}")
    }

    pub fn delay(&self, delay_ms: i64) -> Self {
        let delay_ms_abs = delay_ms.abs() as u64;

        if delay_ms > 0 {
            Self::new(self.inner + Duration::from_millis(delay_ms_abs))
        } else {
            Self::new(self.inner - Duration::from_millis(delay_ms_abs))
        }
    }
}

impl FromStr for Timestamp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = TIMESTAMP_REGEX
            .captures(s)
            .ok_or(Error::InvalidTimestamp(s.to_owned()))?;

        let hours = captures["hours"]
            .parse::<u64>()
            .map_err(|_| Error::InvalidTimestamp(s.to_owned()))?;
        let minutes = captures["minutes"]
            .parse::<u64>()
            .map_err(|_| Error::InvalidTimestamp(s.to_owned()))?;
        let seconds = captures["seconds"]
            .parse::<u64>()
            .map_err(|_| Error::InvalidTimestamp(s.to_owned()))?;
        let milliseconds = captures["milliseconds"]
            .parse::<u64>()
            .map_err(|_| Error::InvalidTimestamp(s.to_owned()))?;

        let timestamp = Self::new(Duration::from_millis(
            milliseconds
                + seconds * MILLISECONDS_PER_SECOND
                + minutes * MILLISECONDS_PER_MINUTE
                + hours * MILLISECONDS_PER_HOUR,
        ));

        Ok(timestamp)
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.components().fmt(f)
    }
}

pub struct TimestampRange {
    start: Timestamp,
    end: Timestamp,
}

impl TimestampRange {
    fn new(start: Timestamp, end: Timestamp) -> Self {
        Self { start, end }
    }

    pub fn as_string(&self) -> String {
        format!("{self}")
    }

    pub fn delay(&self, delay_ms: i64) -> Self {
        Self::new(self.start.delay(delay_ms), self.end.delay(delay_ms))
    }
}

impl FromStr for TimestampRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = TIMESTAMP_RANGE_REGEX
            .captures(s)
            .ok_or(Error::InvalidTimestamp(s.to_owned()))?;

        let start = Timestamp::from_str(&captures["start"])?;
        let end = Timestamp::from_str(&captures["end"])?;

        Ok(Self::new(start, end))
    }
}

impl Display for TimestampRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} --> {}", self.start, self.end)
    }
}
