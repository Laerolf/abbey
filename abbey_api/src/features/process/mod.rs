mod task;
pub use task::Task;

mod cycle;
pub use cycle::Cycle;
use time::OffsetDateTime;

/// Represents a process.
pub trait Process {    
    /// Starts this process.
    fn start(&mut self, now: OffsetDateTime);
}

/// Represents the status of a [`Process`].
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Status {
    /// The process has been created.
    New,
    /// The process has started and is in progress.
    InProgress,
    /// The process has been paused.
    Paused,
    /// The process has been completed.
    Completed
}
