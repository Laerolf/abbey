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

    /// The duration of this task.
    pub duration: Duration
}

impl Task {
    /// Creates a new `Task` based on the provided [`time::Duration`].
    pub fn new(duration: Duration) -> Self {
        Self {
            id: Uuid::new_v4(),
            status: Status::New,
            started_at: None,
            duration,
        }
    }

    /// Gets the end time of this task based on its start time and duration.
    pub fn ends_at(&self) -> Option<OffsetDateTime> {
        match &self.started_at {
            Some(started_at) => started_at.checked_add(self.duration),
            None => None,
        }
    }

    /// Gets the progress of this task.
    pub fn progress(&self, now: OffsetDateTime) -> f32 {
        if self.status != Status::InProgress {
            return 0.0;
        }

        match self.started_at {
            Some(started_at) => {
                let elapsed_duration = now - started_at;
                (elapsed_duration.whole_seconds().max(0) as f32
                    / self.duration.whole_seconds().max(1) as f32)
                    .clamp(0.00, 1.00)
            }
            None => 0.0,
        }
    }
}

impl Process for Task {
    /// Starts this task.
    fn start(&mut self, now: OffsetDateTime) {
        self.status = Status::InProgress;
        self.started_at = Some(now);
    }
}

#[cfg(test)]
mod task_tests {

    mod new_task {
        use time::{Duration, OffsetDateTime};

        use crate::features::process::{Status, Task};

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
    }
}
