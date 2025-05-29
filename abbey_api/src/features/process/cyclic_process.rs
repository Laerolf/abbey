use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use super::{Process, Status};

/// Represents a [`super::Process`] with cycles that have an interval.
pub struct CyclicProcess {
    /// The ID of this cyclic process.
    pub id: Uuid,

    /// The status of this cyclic process.
    pub status: Status,

    /// The time this cyclic process was started last.
    pub started_at: Option<OffsetDateTime>,

    /// The time this cyclic process was paused last.
    pub paused_at: Option<OffsetDateTime>,

    /// The cycle interval of this cyclic process.
    pub cycle_interval: Duration,

    /// The time that has elapsed since this cyclic process was started.
    pub elapsed: Duration,
}

impl CyclicProcess {
    /// Creates a new `CycleProcess` based on the provided cycle [`time::Duration`].
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
}

impl Process for CyclicProcess {
    // Starts this cyclic process.
    fn start(&mut self, now: OffsetDateTime) {
        if !self.can_start() {
            return;
        }

        self.status = Status::InProgress;
        self.started_at = Some(now);
    }

    // Pauses this cyclic process.
    fn pause(&mut self, now: OffsetDateTime) {
        if !self.can_pause() {
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

    /// Resumes this cyclic process.
    fn resume(&mut self, now: OffsetDateTime) {
        if !self.can_resume() {
            return;
        }

        self.started_at = Some(now);
        self.status = Status::InProgress;
        self.paused_at = None;
    }

    /// Can this cyclic process be started?
    fn can_start(&self) -> bool {
        self.status == Status::New
    }

    /// Can this cyclic process be paused?
    fn can_pause(&self) -> bool {
        self.status == Status::InProgress
    }

    /// Can this cyclic process be resumed?
    fn can_resume(&self) -> bool {
        self.status == Status::Paused
    }

    /// Can this cyclic process be completed?
    fn can_complete(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod cyclic_process_tests {

    mod new_cyclic_process {
        use time::{Duration, OffsetDateTime};

        use crate::features::process::{CyclicProcess, Process, Status};

        #[test]
        fn a_cyclic_process_has_an_id() {
            // When
            let cyclic_process = CyclicProcess::new(Duration::minutes(1));

            // Then
            assert!(!cyclic_process.id.to_string().is_empty());
        }

        #[test]
        fn a_new_cyclic_process_is_new() {
            // When
            let cyclic_process = CyclicProcess::new(Duration::minutes(1));

            // Then
            assert_eq!(Status::New, cyclic_process.status);
        }

        #[test]
        fn a_cyclic_process_has_an_duration() {
            // Given
            let cycle_duration = Duration::minutes(1);

            // When
            let cyclic_process = CyclicProcess::new(cycle_duration);

            // Then
            assert_eq!(cycle_duration, cyclic_process.cycle_interval);
        }

        #[test]
        fn a_cyclic_process_has_no_initial_start_time() {
            // When
            let cyclic_process = CyclicProcess::new(Duration::minutes(1));

            // Then
            assert_eq!(None, cyclic_process.started_at);
        }

        #[test]
        fn a_cyclic_process_has_no_initial_completed_cycles() {
            // Given
            let ten_minutes_later = OffsetDateTime::now_utc()
                .checked_add(Duration::minutes(10))
                .expect("10 minutes later is unknown.");

            // When
            let cyclic_process = CyclicProcess::new(Duration::minutes(1));

            // Then
            assert_eq!(0, cyclic_process.completed_cycles(ten_minutes_later));
        }

        #[test]
        fn a_cyclic_process_has_no_initial_elapsed_duration() {
            // When
            let cyclic_process = CyclicProcess::new(Duration::minutes(1));

            // Then
            assert_eq!(Duration::ZERO, cyclic_process.elapsed);
        }

        #[test]
        fn a_cyclic_process_has_no_paused_time() {
            // When
            let cyclic_process = CyclicProcess::new(Duration::minutes(1));

            // Then
            assert_eq!(None, cyclic_process.paused_at);
        }

        #[test]
        fn a_new_cyclic_process_can_not_be_paused() {
            // Given
            let mut cyclic_process = CyclicProcess::new(Duration::minutes(1));

            // When
            cyclic_process.pause(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::New, cyclic_process.status);
        }

        #[test]
        fn a_new_cyclic_process_can_not_be_resumed() {
            // Given
            let mut cyclic_process = CyclicProcess::new(Duration::minutes(1));

            cyclic_process.pause(OffsetDateTime::now_utc());

            // When
            cyclic_process.resume(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::New, cyclic_process.status);
        }

        #[test]
        fn a_cyclic_process_can_not_be_completed() {
            // Given
            let cyclic_process = CyclicProcess::new(Duration::minutes(1));

            // When + then
            assert!(!cyclic_process.can_complete())
        }
    }

    mod started_cyclic_process {
        use time::{Duration, OffsetDateTime};

        use crate::features::process::{CyclicProcess, Process, Status};

        #[test]
        fn a_started_cyclic_process_is_in_progress() {
            // Given
            let mut cyclic_process = CyclicProcess::new(Duration::minutes(1));

            // When
            cyclic_process.start(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::InProgress, cyclic_process.status);
        }

        #[test]
        fn a_started_cyclic_process_has_a_start_time() {
            // Given
            let started_after = OffsetDateTime::now_utc();
            let mut cyclic_process = CyclicProcess::new(Duration::minutes(1));

            // When
            cyclic_process.start(OffsetDateTime::now_utc());

            // Then
            let started_before = OffsetDateTime::now_utc();

            assert!(
                started_after
                    < cyclic_process
                        .started_at
                        .expect("The start time of the cyclic process is unknown.")
            );
            assert!(
                started_before
                    > cyclic_process
                        .started_at
                        .expect("The start time of the cyclic process is unknown.")
            );
        }

        #[test]
        fn a_started_cyclic_process_has_completed_cycles_after_10_minutes() {
            // Given
            let ten_minutes_and_thirty_seconds_later = OffsetDateTime::now_utc()
                .checked_add(Duration::seconds((10 * 60) + 30))
                .expect("10 minutes and 30 seconds later is unknown.");

            let cycle_duration = Duration::minutes(1);
            let mut cyclic_process = CyclicProcess::new(cycle_duration);

            // When
            cyclic_process.start(OffsetDateTime::now_utc());

            // Then
            assert_eq!(
                10,
                cyclic_process.completed_cycles(ten_minutes_and_thirty_seconds_later)
            );
        }

        #[test]
        fn a_started_cyclic_process_can_be_paused() {
            // Given
            let ten_minutes_and_thirty_seconds = Duration::seconds((10 * 60) + 30);
            let now = OffsetDateTime::now_utc();
            let ten_minutes_and_thirty_seconds_later = now
                .checked_add(ten_minutes_and_thirty_seconds)
                .expect("10 minutes and 30 seconds later is unknown.");

            let cycle_duration = Duration::minutes(1);
            let mut cyclic_process = CyclicProcess::new(cycle_duration);

            cyclic_process.start(now);

            // When
            cyclic_process.pause(ten_minutes_and_thirty_seconds_later);

            // Then
            assert_eq!(Status::Paused, cyclic_process.status);
            assert_eq!(ten_minutes_and_thirty_seconds, cyclic_process.elapsed);
        }

        #[test]
        fn a_started_cyclic_process_without_a_start_time_can_not_be_paused() {
            // Given
            let ten_minutes_and_thirty_seconds = Duration::seconds((10 * 60) + 30);
            let now = OffsetDateTime::now_utc();
            let ten_minutes_and_thirty_seconds_later = now
                .checked_add(ten_minutes_and_thirty_seconds)
                .expect("10 minutes and 30 seconds later is unknown.");

            let cycle_duration = Duration::minutes(1);
            let mut cyclic_process = CyclicProcess::new(cycle_duration);

            cyclic_process.start(now);
            cyclic_process.started_at = None;

            // When
            cyclic_process.pause(ten_minutes_and_thirty_seconds_later);

            // Then
            assert_eq!(Status::InProgress, cyclic_process.status);
        }
    }

    mod paused_cyclic_process {
        use time::{Duration, OffsetDateTime};

        use crate::features::process::{CyclicProcess, Process, Status};

        #[test]
        fn a_paused_cyclic_process_is_paused() {
            // Given
            let mut cyclic_process = CyclicProcess::new(Duration::minutes(1));

            cyclic_process.start(OffsetDateTime::now_utc());

            // When
            cyclic_process.pause(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::Paused, cyclic_process.status);
        }

        #[test]
        fn a_paused_cyclic_process_can_not_be_started() {
            // Given
            let mut cyclic_process = CyclicProcess::new(Duration::minutes(1));

            cyclic_process.start(OffsetDateTime::now_utc());
            cyclic_process.pause(OffsetDateTime::now_utc());

            // When
            cyclic_process.start(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::Paused, cyclic_process.status);
        }

        #[test]
        fn a_paused_cyclic_process_has_no_start_time() {
            // Given
            let mut cyclic_process = CyclicProcess::new(Duration::minutes(1));

            cyclic_process.start(OffsetDateTime::now_utc());

            // When
            cyclic_process.pause(OffsetDateTime::now_utc());

            // Then
            assert_eq!(None, cyclic_process.started_at);
        }

        #[test]
        fn a_paused_cyclic_process_can_be_resumed() {
            // Given
            let ten_minutes_and_thirty_seconds = Duration::seconds((10 * 60) + 30);
            let now = OffsetDateTime::now_utc();

            let ten_minutes_and_thirty_seconds_later = now
                .checked_add(ten_minutes_and_thirty_seconds)
                .expect("10 minutes and 30 seconds later is unknown.");
            let twenty_minutes_later = now
                .checked_add(Duration::minutes(20))
                .expect("10 minutes and 30 seconds later is unknown.");

            let mut cyclic_process = CyclicProcess::new(Duration::minutes(1));

            cyclic_process.start(now);
            cyclic_process.pause(ten_minutes_and_thirty_seconds_later);

            // When
            cyclic_process.resume(twenty_minutes_later);

            // Then
            assert_eq!(Status::InProgress, cyclic_process.status);
            assert_eq!(Some(twenty_minutes_later), cyclic_process.started_at);
            assert_eq!(ten_minutes_and_thirty_seconds, cyclic_process.elapsed);
            assert_eq!(None, cyclic_process.paused_at);
        }

        #[test]
        fn a_paused_cyclic_process_keeps_completed_cycles_after_10_minutes() {
            // Given
            let ten_minutes_and_thirty_seconds_later = OffsetDateTime::now_utc()
                .checked_add(Duration::seconds((10 * 60) + 30))
                .expect("10 minutes and 30 seconds later is unknown.");

            let cycle_duration = Duration::minutes(1);
            let mut cyclic_process = CyclicProcess::new(cycle_duration);

            cyclic_process.start(OffsetDateTime::now_utc());

            // When
            cyclic_process.pause(ten_minutes_and_thirty_seconds_later);

            // Then
            assert_eq!(
                10,
                cyclic_process.completed_cycles(ten_minutes_and_thirty_seconds_later)
            );
        }
    }
}
