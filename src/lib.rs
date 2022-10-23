//
// Copyright 2020-2022 Hans W. Uhlig. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Turn Based Simulation Clock
//!
//! Taken From [Hendricksonian Metric Calendar](https://www.broadbandtechreport.com/home/article/16437240/time-for-a-metric-calendar)
//!
//! ## Uses Metric Time increments
//! | Metric Time        | Metric Scale        | Scale      | Seconds                 | Imperial Duration                         |
//! |--------------------|---------------------|------------|-------------------------|-------------------------------------------|
//! | 1 Metric Millennia | 10 Metric Centuries | 1000       | 100,000,000,000 Seconds | 3170 yrs 51 wks 0 days 9 hr 46 min 40 sec |
//! | 1 Metric Century   | 10 Metric Decades   | 100        | 10,000,000,000 Seconds  | 317 yrs 5 wks 0 days 17 hr 46 min 40 sec  |
//! | 1 Metric Decade    | 10 Metric Years     | 10         | 1,000,000,000 Seconds   | 31 yrs 37 wks 0 days 1 hr 46 min 40 sec   |
//! | 1 Metric Year      | 10 Metric Months    | 1          | 100,000,000 Seconds     | 3 yrs 8 wks 6 days 9 hr 46 min 40 sec     |
//! | 1 Metric Month     | 10 Metric Weeks     | 0.1        | 10,000,000 Seconds      | 16 wks 3 days 17 hr 46 min 40 sec         |
//! | 1 Metric Week      | 10 Metric Days      | 0.01       | 1,000,000 Seconds       | 1 wk 4 days 13 hr 46 min 40 sec           |
//! | 1 Metric Day       | 10 Metric Hours     | 0.001      | 100,000 Seconds         | 1 day 3 hr 46 min 40 sec                  |
//! | 1 Metric Hour      | 100 Metric Minutes  | 0.0001     | 10,000 Seconds          | 2 hr 46 min 40 sec                        |
//! | 1 Metric Minute    | 100 Metric Seconds  | 0.000001   | 100 Seconds             | 1 Min 40 Sec                              |
//! | 1 Metric Second    |                     | 0.00000001 | 1 Second                | 1 Second                                  |
//!

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Simulation Clock
///
/// ```rust
/// use std::time::Duration;
/// use metriclock::SimulationClock;
///
/// let mut clock = SimulationClock::default();
///
///clock.tick(Duration::from_micros(50));
/// ```
///
#[derive(Clone, Serialize, Deserialize)]
pub struct SimulationClock {
    /// Seconds since Simulation Epoch
    clock_time: Duration,
    /// Current Clock Mode
    /// * TurnBased
    /// * RealTime
    clock_mode: ClockMode,
    /// Clock Speed Multiplier
    clock_speed: f64,
    /// Duration of a Turn
    turn_duration: Duration,
    /// Duration remaining in this Turn
    turn_time_remaining: Duration,
}

/// Clock Mode
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum ClockMode {
    /// Turn-Based Clock
    TurnBased,
    /// Real-Time Clock
    RealTime,
}

impl SimulationClock {
    /// Create a new Simulation Clock starting at `origin` seconds.
    pub fn from_seconds(epoch_seconds: u64) -> SimulationClock {
        SimulationClock {
            clock_time: Duration::from_secs(epoch_seconds),
            clock_mode: ClockMode::RealTime,
            clock_speed: 0.0,
            turn_duration: Duration::from_secs_f64(3.0),
            turn_time_remaining: Default::default(),
        }
    }
    ///
    pub fn from_metric_timestamp(
        year: u64,
        month: u64,
        week: u64,
        day: u64,
        hour: u64,
        minute: u64,
        second: u64,
    ) -> SimulationClock {
        let mut epoch_seconds = 0;
        epoch_seconds += year * 100_000_000;
        epoch_seconds += month * 10_000_000;
        epoch_seconds += week * 1_000_000;
        epoch_seconds += day * 100_000;
        epoch_seconds += hour * 10_000;
        epoch_seconds += minute * 100;
        epoch_seconds += second * 1;
        let clock_time = Duration::from_secs(epoch_seconds);
        Self {
            clock_time,
            ..Default::default()
        }
    }
    pub fn current_timestamp(&self) -> SimulationTimestamp {
        SimulationTimestamp::from(self.clock_time.clone())
    }
    pub fn current_datetime(&self) -> SimulationDateTime {
        SimulationDateTime::from(self.clock_time.clone())
    }
    pub fn current_epoch_seconds(&self) -> f64 {
        self.clock_time.as_secs_f64()
    }
    pub fn clock_speed(&self) -> f64 {
        self.clock_speed
    }
    pub fn set_clock_speed(&mut self, speed: f64) {
        self.clock_speed = speed;
    }
    pub fn enable_turn_mode(&mut self) {
        if self.clock_mode == ClockMode::RealTime {
            self.clock_mode = ClockMode::TurnBased;
            self.turn_time_remaining = self.turn_duration.clone();
        }
    }
    pub fn turn_complete(&self) -> bool {
        self.turn_time_remaining.is_zero()
    }
    pub fn advance_turn(&mut self) {
        if self.clock_mode == ClockMode::TurnBased
            && self.turn_time_remaining == Duration::default()
        {
            self.turn_time_remaining = self.turn_duration.clone();
        }
    }
    pub fn disable_turn_mode(&mut self) {
        if self.clock_mode == ClockMode::TurnBased {
            self.clock_mode = ClockMode::RealTime;
            self.turn_time_remaining = Duration::default();
        }
    }
    pub fn tick(&mut self, delta: Duration) {
        let delta = delta.mul_f64(self.clock_speed.into());
        match self.clock_mode {
            ClockMode::RealTime => {
                self.clock_time += delta;
            }
            ClockMode::TurnBased => {
                if !self.turn_time_remaining.is_zero() {
                    self.turn_time_remaining = self.turn_time_remaining.saturating_sub(delta);
                    self.clock_time += delta;
                }
            }
        }
    }
}

impl Default for SimulationClock {
    fn default() -> SimulationClock {
        SimulationClock {
            clock_time: Default::default(),
            clock_mode: ClockMode::RealTime,
            clock_speed: 1.0,
            turn_duration: Duration::from_secs(6),
            turn_time_remaining: Duration::default(),
        }
    }
}

impl std::fmt::Debug for SimulationClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimulationClock")
            .field("clock_epoch_seconds", &self.current_epoch_seconds())
            .field("clock_timestamp", &self.current_timestamp())
            .field("clock_datetime", &self.current_datetime())
            .field("clock_speed", &self.clock_speed)
            .field("clock_state", &self.clock_mode)
            .field("turn_time", &self.turn_duration)
            .field("turn_remaining", &self.turn_time_remaining)
            .finish()
    }
}

/// Fixed Timestamp
#[derive(Clone, Serialize, Deserialize)]
pub struct SimulationTimestamp(Duration);

impl SimulationTimestamp {
    pub fn from_epoch_seconds(epoch_seconds: u64) -> SimulationTimestamp {
        Self(Duration::from_secs(epoch_seconds))
    }
    pub fn from_components(
        year: u32,
        month: u8,
        week: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> SimulationTimestamp {
        let mut epoch_seconds = 0.0;
        epoch_seconds += year as f64 * 100_000_000.0;
        epoch_seconds += month as f64 * 10_000_000.0;
        epoch_seconds += week as f64 * 1_000_000.0;
        epoch_seconds += day as f64 * 100_000.0;
        epoch_seconds += hour as f64 * 10_000.0;
        epoch_seconds += minute as f64 * 100.0;
        epoch_seconds += second as f64 * 1.0;
        Self(Duration::from_secs_f64(epoch_seconds))
    }
}

impl From<Duration> for SimulationTimestamp {
    fn from(duration: Duration) -> Self {
        SimulationTimestamp(duration)
    }
}

impl From<SimulationDateTime> for SimulationTimestamp {
    fn from(datetime: SimulationDateTime) -> Self {
        let mut epoch_seconds = 0;
        epoch_seconds += datetime.year as u64 * 100_000_000;
        epoch_seconds += datetime.month as u64 * 10_000_000;
        epoch_seconds += datetime.week as u64 * 1_000_000;
        epoch_seconds += datetime.day as u64 * 100_000;
        epoch_seconds += datetime.hour as u64 * 10_000;
        epoch_seconds += datetime.minute as u64 * 100;
        epoch_seconds += datetime.second as u64 * 1;
        Self(Duration::from_secs(epoch_seconds as u64))
    }
}

impl std::fmt::Debug for SimulationTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimulationTimestamp")
            .field("epoch_seconds", &self.0.as_secs())
            .finish()
    }
}

impl std::fmt::Display for SimulationTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_secs_f64())
    }
}

/// Data Time of the Simulation
pub struct SimulationDateTime {
    pub year: u32,
    pub month: u8,
    pub week: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl SimulationDateTime {
    pub fn from_epoch_seconds(epoch_seconds: u64) -> Self {
        let mut epoch_seconds = epoch_seconds;
        let year = epoch_seconds / 100_000_000;
        epoch_seconds -= year * 100_000_000;
        let month = epoch_seconds / 10_000_000;
        epoch_seconds -= month * 10_000_000;
        let week = epoch_seconds / 1_000_000;
        epoch_seconds -= week * 1_000_000;
        let day = epoch_seconds / 100_000;
        epoch_seconds -= day * 100_000;
        let hour = epoch_seconds / 10_000;
        epoch_seconds -= hour * 10_000;
        let minute = epoch_seconds / 100;
        epoch_seconds -= minute * 100;
        let second = epoch_seconds;
        Self {
            year: year as u32,
            month: month as u8,
            week: week as u8,
            day: day as u8,
            hour: hour as u8,
            minute: minute as u8,
            second: second as u8,
        }
    }
    pub fn from_components(
        year: u32,
        month: u8,
        week: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> SimulationDateTime {
        SimulationDateTime {
            year,
            month,
            week,
            day,
            hour,
            minute,
            second,
        }
    }
}

impl From<Duration> for SimulationDateTime {
    fn from(duration: Duration) -> Self {
        SimulationDateTime::from_epoch_seconds(duration.as_secs())
    }
}

impl From<SimulationTimestamp> for SimulationDateTime {
    fn from(timestamp: SimulationTimestamp) -> Self {
        SimulationDateTime::from_epoch_seconds(timestamp.0.as_secs())
    }
}

impl std::fmt::Display for SimulationDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{:02}-{:02}-{:02}@{:02}:{:02}:{:02.4}",
            &self.year, &self.month, &self.week, &self.day, &self.hour, &self.minute, &self.second
        )
    }
}

impl std::fmt::Debug for SimulationDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimulationDateTime")
            .field("year", &self.year)
            .field("month", &self.month)
            .field("week", &self.week)
            .field("month", &self.month)
            .field("day", &self.day)
            .field("hour", &self.hour)
            .field("minute", &self.minute)
            .field("second", &self.second)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::SimulationClock;
    use std::time::Duration;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn clock_test() {
        let mut clock = SimulationClock::default();
        let mut seconds_since_startup = Duration::default();
        let mut seconds_since_update = Duration::default();
        while seconds_since_startup.as_secs_f64() < 20.0 {
            seconds_since_startup += Duration::from_secs_f64(1.0 / 60.0);
            seconds_since_update += Duration::from_secs_f64(1.0 / 60.0);
            clock.tick(seconds_since_update);
        }
        clock.enable_turn_mode();
        while seconds_since_startup.as_secs_f64() < 40.0 {
            seconds_since_startup += Duration::from_secs_f64(1.0 / 60.0);
            seconds_since_update += Duration::from_secs_f64(1.0 / 60.0);
            clock.tick(seconds_since_update);
            if clock.turn_complete() {
                clock.advance_turn();
            }
        }
        clock.disable_turn_mode();
        while seconds_since_startup.as_secs_f64() < 60.0 {
            seconds_since_startup += Duration::from_secs_f64(1.0 / 60.0);
            seconds_since_update += Duration::from_secs_f64(1.0 / 60.0);
            clock.tick(seconds_since_update);
        }
    }
}
