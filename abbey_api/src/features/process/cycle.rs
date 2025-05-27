use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use super::{Process, Status};

/// Represents a [`super::Process`] with cycles that have an interval.
pub struct Cycle {
    /// The ID of this process.
    pub id: Uuid,

    /// The status of this process.
    pub status: Status,

    /// The time this process was started last.
    pub started_at: Option<OffsetDateTime>,

    /// The time this process was paused last.
    pub paused_at: Option<OffsetDateTime>,

    /// The cycle interval of this process.
    pub cycle_interval: Duration,

    /// The time that has elapsed since the process was started.
    pub elapsed: Duration,
}

impl Cycle {
    /// Creates a new `Cycle` based on the provided cycle [`time::Duration`].
    pub fn new(cycle_interval: Duration) -> Self {
        Self {
            id: Uuid::new_v4(),
            status: Status::New,
            started_at: None,
            paused_at: None,
            cycle_interval,
            elapsed: Duration::ZERO,
        }
    }

    /// Calculates the completed cycles since the provided time.
    pub fn completed_cycles(&self, last_cycle_check: OffsetDateTime) -> u32 {
        let cycle_elapsed_duration: Duration = match (self.started_at, self.status) {
            (Some(started_at), Status::InProgress) => {
                self.elapsed + (last_cycle_check - started_at)
            }
            (_, Status::Paused) => self.elapsed,
            _ => Duration::ZERO,
        };

        (cycle_elapsed_duration.as_seconds_f32() / self.cycle_interval.as_seconds_f32()).floor()
            as u32
    }

    /// Stops this process.
    pub fn pause(&mut self, now: OffsetDateTime) {
        if self.status != Status::InProgress {
            return;
        }

        if let Some(started_at) = self.started_at {
            let cycle_duration = now - started_at;
            self.elapsed += cycle_duration;
            self.paused_at = Some(now);
            self.status = Status::Paused;
            self.started_at = None;
        }
    }

    /// Resumes this process if it would be paused.
    pub fn resume(&mut self, now: OffsetDateTime) {
        if self.status != Status::Paused {
            return;
        }

        self.started_at = Some(now);
        self.status = Status::InProgress;
        self.paused_at = None;
    }
}

impl Process for Cycle {
    /// Starts this cycle process.
    fn start(&mut self, now: OffsetDateTime) {
        if self.status != Status::New {
            return;
        }

        self.status = Status::InProgress;
        self.started_at = Some(now);
    }
}

#[cfg(test)]
mod cycle_tests {

    mod new_cycle {
        use time::{Duration, OffsetDateTime};

        use crate::features::process::{Cycle, Status};

        #[test]
        fn a_cycle_has_an_id() {
            // When
            let cycle = Cycle::new(Duration::minutes(1));

            // Then
            assert!(!cycle.id.to_string().is_empty());
        }

        #[test]
        fn a_new_cycle_is_new() {
            // When
            let cycle = Cycle::new(Duration::minutes(1));

            // Then
            assert_eq!(Status::New, cycle.status);
        }

        #[test]
        fn a_cycle_has_an_duration() {
            // Given
            let cycle_duration = Duration::minutes(1);

            // When
            let cycle = Cycle::new(cycle_duration);

            // Then
            assert_eq!(cycle_duration, cycle.cycle_interval);
        }

        #[test]
        fn a_cycle_has_no_initial_start_time() {
            // When
            let cycle = Cycle::new(Duration::minutes(1));

            // Then
            assert_eq!(None, cycle.started_at);
        }

        #[test]
        fn a_cycle_has_no_initial_completed_cycles() {
            // Given
            let ten_minutes_later = OffsetDateTime::now_utc()
                .checked_add(Duration::minutes(10))
                .expect("10 minutes later is unknown.");

            // When
            let cycle = Cycle::new(Duration::minutes(1));

            // Then
            assert_eq!(0, cycle.completed_cycles(ten_minutes_later));
        }

        #[test]
        fn a_cycle_has_no_initial_elapsed_duration() {
            // When
            let cycle = Cycle::new(Duration::minutes(1));

            // Then
            assert_eq!(Duration::ZERO, cycle.elapsed);
        }

        #[test]
        fn a_cycle_has_no_paused_time() {
            // When
            let cycle = Cycle::new(Duration::minutes(1));

            // Then
            assert_eq!(None, cycle.paused_at);
        }

        #[test]
        fn a_new_cycle_can_not_be_paused() {
            // Given
            let mut cycle = Cycle::new(Duration::minutes(1));

            // When
            cycle.pause(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::New, cycle.status);
        }

        #[test]
        fn a_new_cycle_can_not_be_resumed() {
            // Given
            let mut cycle = Cycle::new(Duration::minutes(1));

            cycle.pause(OffsetDateTime::now_utc());

            // When
            cycle.resume(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::New, cycle.status);
        }
    }

    mod started_cycle {
        use time::{Duration, OffsetDateTime};

        use crate::features::process::{Cycle, Process, Status};

        #[test]
        fn a_started_cycle_is_in_progress() {
            // Given
            let mut cycle = Cycle::new(Duration::minutes(1));

            // When
            cycle.start(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::InProgress, cycle.status);
        }

        #[test]
        fn a_started_cycle_has_a_start_time() {
            // Given
            let started_after = OffsetDateTime::now_utc();
            let mut cycle = Cycle::new(Duration::minutes(1));

            // When
            cycle.start(OffsetDateTime::now_utc());

            // Then
            let started_before = OffsetDateTime::now_utc();

            assert!(
                started_after
                    < cycle
                        .started_at
                        .expect("The start time of the cycle is unknown.")
            );
            assert!(
                started_before
                    > cycle
                        .started_at
                        .expect("The start time of the cycle is unknown.")
            );
        }

        #[test]
        fn a_started_cycle_has_completed_cycles_after_10_minutes() {
            // Given
            let ten_minutes_and_thirty_seconds_later = OffsetDateTime::now_utc()
                .checked_add(Duration::seconds((10 * 60) + 30))
                .expect("10 minutes and 30 seconds later is unknown.");

            let cycle_duration = Duration::minutes(1);
            let mut cycle = Cycle::new(cycle_duration);

            // When
            cycle.start(OffsetDateTime::now_utc());

            // Then
            assert_eq!(
                10,
                cycle.completed_cycles(ten_minutes_and_thirty_seconds_later)
            );
        }

        #[test]
        fn a_started_cycle_can_be_paused() {
            // Given
            let ten_minutes_and_thirty_seconds = Duration::seconds((10 * 60) + 30);
            let now = OffsetDateTime::now_utc();
            let ten_minutes_and_thirty_seconds_later = now
                .checked_add(ten_minutes_and_thirty_seconds)
                .expect("10 minutes and 30 seconds later is unknown.");

            let cycle_duration = Duration::minutes(1);
            let mut cycle = Cycle::new(cycle_duration);

            cycle.start(now);

            // When
            cycle.pause(ten_minutes_and_thirty_seconds_later);

            // Then
            assert_eq!(Status::Paused, cycle.status);
            assert_eq!(ten_minutes_and_thirty_seconds, cycle.elapsed);
        }

        #[test]
        fn a_started_cycle_without_a_start_time_can_not_be_paused() {
            // Given
            let ten_minutes_and_thirty_seconds = Duration::seconds((10 * 60) + 30);
            let now = OffsetDateTime::now_utc();
            let ten_minutes_and_thirty_seconds_later = now
                .checked_add(ten_minutes_and_thirty_seconds)
                .expect("10 minutes and 30 seconds later is unknown.");

            let cycle_duration = Duration::minutes(1);
            let mut cycle = Cycle::new(cycle_duration);

            cycle.start(now);
            cycle.started_at = None;

            // When
            cycle.pause(ten_minutes_and_thirty_seconds_later);

            // Then
            assert_eq!(Status::InProgress, cycle.status);
        }
    }

    mod paused_cycle {
        use time::{Duration, OffsetDateTime};

        use crate::features::process::{Cycle, Process, Status};

        #[test]
        fn a_paused_cycle_is_paused() {
            // Given
            let mut cycle = Cycle::new(Duration::minutes(1));

            cycle.start(OffsetDateTime::now_utc());

            // When
            cycle.pause(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::Paused, cycle.status);
        }

        #[test]
        fn a_paused_cycle_can_not_be_started() {
            // Given
            let mut cycle = Cycle::new(Duration::minutes(1));

            cycle.start(OffsetDateTime::now_utc());
            cycle.pause(OffsetDateTime::now_utc());

            // When
            cycle.start(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::Paused, cycle.status);
        }

        #[test]
        fn a_paused_cycle_has_no_start_time() {
            // Given
            let mut cycle = Cycle::new(Duration::minutes(1));

            cycle.start(OffsetDateTime::now_utc());

            // When
            cycle.pause(OffsetDateTime::now_utc());

            // Then
            assert_eq!(None, cycle.started_at);
        }

        #[test]
        fn a_paused_cycle_can_be_resumed() {
            // Given
            let ten_minutes_and_thirty_seconds = Duration::seconds((10 * 60) + 30);
            let now = OffsetDateTime::now_utc();

            let ten_minutes_and_thirty_seconds_later = now
                .checked_add(ten_minutes_and_thirty_seconds)
                .expect("10 minutes and 30 seconds later is unknown.");
            let twenty_minutes_later = now
                .checked_add(Duration::minutes(20))
                .expect("10 minutes and 30 seconds later is unknown.");

            let mut cycle = Cycle::new(Duration::minutes(1));

            cycle.start(now);
            cycle.pause(ten_minutes_and_thirty_seconds_later);

            // When
            cycle.resume(twenty_minutes_later);

            // Then
            assert_eq!(Status::InProgress, cycle.status);
            assert_eq!(Some(twenty_minutes_later), cycle.started_at);
            assert_eq!(ten_minutes_and_thirty_seconds, cycle.elapsed);
            assert_eq!(None, cycle.paused_at);
        }

        #[test]
        fn a_paused_cycle_keeps_completed_cycles_after_10_minutes() {
            // Given
            let ten_minutes_and_thirty_seconds_later = OffsetDateTime::now_utc()
                .checked_add(Duration::seconds((10 * 60) + 30))
                .expect("10 minutes and 30 seconds later is unknown.");

            let cycle_duration = Duration::minutes(1);
            let mut cycle = Cycle::new(cycle_duration);

            cycle.start(OffsetDateTime::now_utc());

            // When
            cycle.pause(ten_minutes_and_thirty_seconds_later);

            // Then
            assert_eq!(
                10,
                cycle.completed_cycles(ten_minutes_and_thirty_seconds_later)
            );
        }
    }
}
