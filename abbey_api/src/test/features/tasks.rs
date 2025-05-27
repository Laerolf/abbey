use time::Duration;

use crate::features::process::Task;

pub struct TaskDataBuilder {
    duration: Duration
}

impl Default for TaskDataBuilder {
    fn default() -> Self {
        Self {
            duration: Duration::minutes(5)
        }
    }
}

impl TaskDataBuilder {
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn build(self) -> Task {
        Task::new(self.duration)
    }
}
