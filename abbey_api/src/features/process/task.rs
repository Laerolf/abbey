use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use super::{Process, Status};

/// Represents a [`super::Process`] that starts at a certain time and ends after a [`time::Duration`] has passed.
pub struct Task {
    /// The ID of this task.
    pub id: Uuid,

    /// The status of this task.
    pub status: Status,

    /// The time this task started.
    pub started_at: Option<OffsetDateTime>,

    /// The time this task was paused.
    pub paused_at: Option<OffsetDateTime>,

    /// The duration of this task.
    pub duration: Duration,

    /// The time that has elapsed since the task was started.
    pub elapsed: Duration,
}

impl Task {
    /// Creates a new `Task` based on the provided [`time::Duration`].
    pub fn new(duration: Duration) -> Self {
        Self {
            id: Uuid::new_v4(),
            status: Status::New,
            started_at: None,
            paused_at: None,
            duration,
            elapsed: Duration::ZERO,
        }
    }

    /// Gets the end time of this task based on its start time, duration and the elapsed time since this task was started.
    pub fn ends_at(&self) -> Option<OffsetDateTime> {
        match &self.started_at {
            Some(started_at) => started_at
                .checked_add(self.duration)
                .expect(
                    "The end time of this task is unknown after adding the duration of the task.",
                )
                .checked_sub(self.elapsed),
            None => None,
        }
    }

    /// Gets the progress of this task.
    pub fn progress(&self, now: OffsetDateTime) -> f32 {
        let task_elapsed_duration: Duration = match (self.started_at, self.status) {
            (Some(started_at), Status::InProgress) => self.elapsed + (now - started_at),
            (_, Status::Paused) => self.elapsed,
            _ => Duration::ZERO,
        };

        (task_elapsed_duration.as_seconds_f32() / self.duration.as_seconds_f32()).clamp(0.00, 1.00)
    }
}

impl Process for Task {
    /// Starts this task.
    fn start(&mut self, now: OffsetDateTime) {
        if !self.can_start() {
            return;
        }

        self.status = Status::InProgress;
        self.started_at = Some(now);
    }

    /// Pauses this task.
    fn pause(&mut self, now: OffsetDateTime) {
        if !self.can_pause() {
            return;
        }

        if let Some(started_at) = self.started_at {
            let task_duration = now - started_at;
            self.elapsed += task_duration;
            self.paused_at = Some(now);
            self.status = Status::Paused;
            self.started_at = None;
        }
    }

    /// Resumes this task.
    fn resume(&mut self, now: OffsetDateTime) {
        if !self.can_resume() {
            return;
        }

        self.started_at = Some(now);
        self.status = Status::InProgress;
        self.paused_at = None;
    }

    /// Can this task be started?
    fn can_start(&self) -> bool {
        self.status == Status::New
    }

    /// Can this task be paused?
    fn can_pause(&self) -> bool {
        self.status == Status::InProgress
    }

    /// Can this task be resumed?
    fn can_resume(&self) -> bool {
        self.status == Status::Paused
    }
}

#[cfg(test)]
mod task_tests {

    mod new_task {
        use time::{Duration, OffsetDateTime};

        use crate::features::process::{Process, Status, Task};

        #[test]
        fn a_task_has_an_id() {
            // When
            let task = Task::new(Duration::minutes(1));

            // Then
            assert!(!task.id.to_string().is_empty());
        }

        #[test]
        fn a_new_task_is_new() {
            // When
            let task = Task::new(Duration::minutes(1));

            // Then
            assert_eq!(Status::New, task.status);
        }

        #[test]
        fn a_task_has_no_initial_start_time() {
            // When
            let task = Task::new(Duration::minutes(1));

            // Then
            assert_eq!(None, task.started_at);
        }

        #[test]
        fn a_task_has_a_duration() {
            // Given
            let expected_duration = Duration::minutes(1);

            // When
            let task = Task::new(expected_duration);

            // Then
            assert_eq!(expected_duration, task.duration);
        }

        #[test]
        fn a_task_has_no_initial_elapsed_time() {
            // When
            let task = Task::new(Duration::minutes(1));

            // Then
            assert_eq!(Duration::ZERO, task.elapsed);
        }

        #[test]
        fn a_task_has_no_initial_paused_time() {
            // When
            let task = Task::new(Duration::minutes(1));

            // Then
            assert_eq!(None, task.paused_at);
        }

        #[test]
        fn a_task_has_no_initial_end_time() {
            // When
            let task = Task::new(Duration::minutes(1));

            // Then
            assert_eq!(None, task.ends_at());
        }

        #[test]
        fn a_task_has_initially_zero_progress() {
            // When
            let task = Task::new(Duration::minutes(1));

            // Then
            assert_eq!(0.0, task.progress(OffsetDateTime::now_utc()));
        }

        #[test]
        fn a_new_task_can_not_be_paused() {
            // Given
            let mut task = Task::new(Duration::minutes(1));

            // When
            task.pause(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::New, task.status);
        }

        #[test]
        fn a_new_task_can_not_be_resumed() {
            // Given
            let mut task = Task::new(Duration::minutes(1));

            // When
            task.resume(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::New, task.status);
        }
    }

    mod started_task {

        use time::{Duration, OffsetDateTime};

        use crate::features::process::{Process, Status, Task};

        #[test]
        fn a_started_task_is_in_progress() {
            // Given
            let mut task = Task::new(Duration::minutes(1));

            // When
            task.start(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::InProgress, task.status);
        }

        #[test]
        fn a_started_task_has_a_start_time() {
            // Given
            let started_after = OffsetDateTime::now_utc();
            let mut task = Task::new(Duration::minutes(1));

            // When
            task.start(OffsetDateTime::now_utc());

            // Then
            let started_before = OffsetDateTime::now_utc();

            assert!(
                started_after
                    < task
                        .started_at
                        .expect("The start time of the task is unknown.")
            );
            assert!(
                started_before
                    > task
                        .started_at
                        .expect("The start time of the task is unknown.")
            );
        }

        #[test]
        fn a_started_task_has_an_end_time() {
            // Given
            let setup_time = OffsetDateTime::now_utc();
            let duration = Duration::minutes(1);
            let mut task = Task::new(duration);

            // When
            task.start(OffsetDateTime::now_utc());

            // Then
            assert!(setup_time.checked_add(duration) < task.ends_at());
            assert!(OffsetDateTime::now_utc().checked_add(duration) > task.ends_at());
        }

        #[test]
        fn a_started_task_has_progress() {
            // Given
            let now = OffsetDateTime::now_utc();
            let timeout: OffsetDateTime = now
                .checked_add(Duration::seconds(30))
                .expect("The timeout time is unknown.");

            let mut task = Task::new(Duration::minutes(1));

            // When
            task.start(now);

            // Then
            let task_progress = task.progress(timeout);

            assert!(task_progress > 0.4);
            assert!(task_progress < 0.6);
        }

        #[test]
        fn a_started_task_without_a_start_date_has_no_progress() {
            // Given
            let now = OffsetDateTime::now_utc();
            let timeout: OffsetDateTime = now
                .checked_add(Duration::seconds(30))
                .expect("The timeout time is unknown.");

            let mut task = Task::new(Duration::minutes(1));

            task.start(now);

            // When
            task.started_at = None;

            // Then
            assert_eq!(0.0, task.progress(timeout));
        }

        #[test]
        fn a_started_task_can_be_paused() {
            // Given
            let mut task = Task::new(Duration::minutes(1));

            task.start(OffsetDateTime::now_utc());

            // When
            task.pause(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::Paused, task.status);
        }

        #[test]
        fn a_started_task_without_a_start_time_can_not_be_paused() {
            // Given
            let mut task = Task::new(Duration::minutes(1));

            task.start(OffsetDateTime::now_utc());
            task.started_at = None;

            // When
            task.pause(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::InProgress, task.status);
        }
    }

    mod paused_task {
        use time::{Duration, OffsetDateTime};

        use crate::features::process::{Process, Status, Task};

        #[test]
        fn a_paused_task_has_elapsed_time() {
            // Given
            let now = OffsetDateTime::now_utc();
            let thirty_seconds = Duration::seconds(30);
            let thirty_seconds_later: OffsetDateTime = now
                .checked_add(thirty_seconds)
                .expect("The timeout time is unknown.");

            let mut task = Task::new(Duration::minutes(1));

            task.start(now);

            // When
            task.pause(thirty_seconds_later);

            // Then
            assert_eq!(thirty_seconds, task.elapsed);
        }

        #[test]
        fn a_paused_task_has_progress() {
            // Given
            let now = OffsetDateTime::now_utc();
            let thirty_seconds_later: OffsetDateTime = now
                .checked_add(Duration::seconds(30))
                .expect("The timeout time is unknown.");

            let mut task = Task::new(Duration::minutes(1));

            task.start(now);

            // When
            task.pause(thirty_seconds_later);

            // Then
            assert!(task.progress(thirty_seconds_later) > 0.4);
            assert!(task.progress(thirty_seconds_later) < 0.6);
        }

        #[test]
        fn a_paused_task_has_no_start_time() {
            // Given
            let mut task = Task::new(Duration::minutes(1));

            task.start(OffsetDateTime::now_utc());

            // When
            task.pause(OffsetDateTime::now_utc());

            // Then
            assert_eq!(None, task.started_at);
        }

        #[test]
        fn a_paused_task_can_not_be_started() {
            // Given
            let mut task = Task::new(Duration::minutes(1));

            task.start(OffsetDateTime::now_utc());
            task.pause(OffsetDateTime::now_utc());

            // When
            task.start(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::Paused, task.status);
        }

        #[test]
        fn a_paused_task_can_be_resumed() {
            // Given
            let mut task = Task::new(Duration::minutes(1));

            task.start(OffsetDateTime::now_utc());
            task.pause(OffsetDateTime::now_utc());

            // When
            task.resume(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::InProgress, task.status);
        }
    }
}
