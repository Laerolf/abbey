use std::{cell::RefCell, rc::Rc};

use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::{
    features::{
        actor::domain::person::Person, output::domain::Output, process::error::ProcessError,
    },
    shared::error::DomainError,
};

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

    /// The people assigned to this task.
    pub assigned_people: Vec<Rc<RefCell<dyn Person>>>,
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
            assigned_people: Vec::new(),
        }
    }

    /// Gets the end time of this task based on its start time, duration, the elapsed time since this task was started and the current stime.
    pub fn ends_at(&self, now: OffsetDateTime) -> Option<OffsetDateTime> {
        match (self.status, self.started_at) {
            (Status::InProgress, Some(started_at)) => started_at
                .checked_add(self.duration)
                .and_then(|ends_at| ends_at.checked_sub(self.elapsed)),
            (Status::Paused, _) => Some(now + (self.duration - self.elapsed)),
            _ => None,
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
    /// Assigns a person to this task.
    fn assign_person(&mut self, person: Rc<RefCell<dyn Person>>) {
        self.assigned_people.push(person);
    }

    /// Unassigns a person to this task.
    fn unassign_person(&mut self, person: &Rc<RefCell<dyn Person>>) {
        self.assigned_people.retain(|p| !Rc::ptr_eq(p, person));
    }

    /// Gets the status of this task.
    fn status(&self) -> Status {
        self.status
    }

    /// Starts this task.
    fn start(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>> {
        if self.status != Status::New {
            return Err(Box::new(ProcessError::NotNew));
        } else if self.assigned_people.is_empty() {
            return Err(Box::new(ProcessError::NoAssignedPeople));
        }

        self.status = Status::InProgress;
        self.started_at = Some(now);
        Ok(())
    }

    /// Pauses this task.
    fn pause(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>> {
        if self.status != Status::InProgress {
            return Err(Box::new(ProcessError::NotInProgress));
        }

        if let Some(started_at) = self.started_at {
            let task_duration = now - started_at;
            self.elapsed += task_duration;
            self.paused_at = Some(now);
            self.status = Status::Paused;
            self.started_at = None;
        }

        Ok(())
    }

    /// Resumes this task.
    fn resume(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>> {
        if self.status != Status::Paused {
            return Err(Box::new(ProcessError::NotPaused));
        } else if self.assigned_people.is_empty() {
            return Err(Box::new(ProcessError::NoAssignedPeople));
        }

        self.started_at = Some(now);
        self.status = Status::InProgress;
        self.paused_at = None;
        Ok(())
    }

    /// Completes this task if possible.
    fn complete(&mut self, now: OffsetDateTime) -> Result<(), Box<dyn DomainError>> {
        if self.status != Status::New {
            return Err(Box::new(ProcessError::NotNew));
        } else if self.assigned_people.is_empty() {
            return Err(Box::new(ProcessError::NoAssignedPeople));
        }

        if self.progress(now) >= 1.0 {
            self.status = Status::Completed;
            self.started_at = None;
            self.paused_at = None;
            Ok(())
        } else {
            Err(Box::new(ProcessError::NotComplete))
        }
    }

    /// Gets the output of this task.
    fn get_yield(&self) -> Option<Output> {
        None
    }
}

#[cfg(test)]
mod task_tests {

    mod new_task {
        use time::{Duration, OffsetDateTime};

        use crate::features::process::domain::{Process, Status, Task};

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
            assert_eq!(None, task.ends_at(OffsetDateTime::now_utc()));
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
            let _ = task.pause(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::New, task.status);
        }

        #[test]
        fn a_new_task_can_not_be_resumed() {
            // Given
            let mut task = Task::new(Duration::minutes(1));

            // When
            let _ = task.resume(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::New, task.status);
        }
    }

    mod started_task {

        use time::{Duration, OffsetDateTime};

        use crate::features::process::domain::{Process, Status, Task};

        #[test]
        fn a_started_task_is_in_progress() {
            // Given
            let mut task = Task::new(Duration::minutes(1));

            // When
            let _ = task.start(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::InProgress, task.status);
        }

        #[test]
        fn a_started_task_has_a_start_time() {
            // Given
            let started_after = OffsetDateTime::now_utc();
            let mut task = Task::new(Duration::minutes(1));

            // When
            let _ = task.start(OffsetDateTime::now_utc());

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
            let _ = task.start(OffsetDateTime::now_utc());

            // Then
            assert!(setup_time.checked_add(duration) < task.ends_at(OffsetDateTime::now_utc()));
            assert!(
                OffsetDateTime::now_utc().checked_add(duration)
                    > task.ends_at(OffsetDateTime::now_utc())
            );
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
            let _ = task.start(now);

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

            let _ = task.start(now);

            // When
            task.started_at = None;

            // Then
            assert_eq!(0.0, task.progress(timeout));
        }

        #[test]
        fn a_started_task_can_be_paused() {
            // Given
            let mut task = Task::new(Duration::minutes(1));

            let _ = task.start(OffsetDateTime::now_utc());

            // When
            let _ = task.pause(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::Paused, task.status);
        }

        #[test]
        fn a_started_task_without_a_start_time_can_not_be_paused() {
            // Given
            let mut task = Task::new(Duration::minutes(1));

            let _ = task.start(OffsetDateTime::now_utc());
            task.started_at = None;

            // When
            let _ = task.pause(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::InProgress, task.status);
        }

        #[test]
        fn a_started_task_with_progress_1_can_be_completed() {
            // Given
            let now = OffsetDateTime::now_utc();
            let one_minute = Duration::minutes(1);
            let one_minute_later = now
                .checked_add(one_minute)
                .expect("One minute later is unknown.");

            let mut task = Task::new(one_minute);

            let _ = task.start(now);

            // When
            let result = task.complete(one_minute_later);

            // Then
            assert!(result.is_ok());
            assert_eq!(Status::Completed, task.status);
            assert_eq!(None, task.started_at);
            assert_eq!(None, task.paused_at);
        }
    }

    mod paused_task {
        use time::{Duration, OffsetDateTime};

        use crate::features::process::domain::{Process, Status, Task};

        #[test]
        fn a_paused_task_has_elapsed_time() {
            // Given
            let now = OffsetDateTime::now_utc();
            let thirty_seconds = Duration::seconds(30);
            let thirty_seconds_later: OffsetDateTime = now
                .checked_add(thirty_seconds)
                .expect("The timeout time is unknown.");

            let mut task = Task::new(Duration::minutes(1));

            let _ = task.start(now);

            // When
            let _ = task.pause(thirty_seconds_later);

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

            let _ = task.start(now);

            // When
            let _ = task.pause(thirty_seconds_later);

            // Then
            assert!(task.progress(thirty_seconds_later) > 0.4);
            assert!(task.progress(thirty_seconds_later) < 0.6);
        }

        #[test]
        fn a_paused_task_has_an_end_time() {
            // Given
            let now = OffsetDateTime::now_utc();
            let thirty_seconds_later: OffsetDateTime = now
                .checked_add(Duration::seconds(30))
                .expect("The timeout time is unknown.");

            let task_duration = Duration::minutes(1);
            let mut task = Task::new(task_duration);

            let _ = task.start(now);

            // When
            let _ = task.pause(thirty_seconds_later);

            // Then
            assert!(
                Some(OffsetDateTime::now_utc() + task_duration)
                    > task.ends_at(OffsetDateTime::now_utc())
            );
        }

        #[test]
        fn a_paused_task_has_no_start_time() {
            // Given
            let mut task = Task::new(Duration::minutes(1));

            let _ = task.start(OffsetDateTime::now_utc());

            // When
            let _ = task.pause(OffsetDateTime::now_utc());

            // Then
            assert_eq!(None, task.started_at);
        }

        #[test]
        fn a_paused_task_can_not_be_started() {
            // Given
            let mut task = Task::new(Duration::minutes(1));

            let _ = task.start(OffsetDateTime::now_utc());
            let _ = task.pause(OffsetDateTime::now_utc());

            // When
            let _ = task.start(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::Paused, task.status);
        }

        #[test]
        fn a_paused_task_can_be_resumed() {
            // Given
            let mut task = Task::new(Duration::minutes(1));

            let _ = task.start(OffsetDateTime::now_utc());
            let _ = task.pause(OffsetDateTime::now_utc());

            // When
            let _ = task.resume(OffsetDateTime::now_utc());

            // Then
            assert_eq!(Status::InProgress, task.status);
        }
    }
}
