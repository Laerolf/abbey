mod task;
pub use task::Task;

mod cyclic_process;
pub use cyclic_process::CyclicProcess;
use time::OffsetDateTime;

/// Represents a process.
pub trait Process {
    /// Starts this process.
    fn start(&mut self, now: OffsetDateTime);

    /// Pauses this process.
    fn pause(&mut self, now: OffsetDateTime);

    /// Resumes this process.
    fn resume(&mut self, now: OffsetDateTime);

    /// Can this process be started?
    fn can_start(&self) -> bool;

    /// Can this process be paused?
    fn can_pause(&self) -> bool;

    /// Can this process be resumed?
    fn can_resume(&self) -> bool;

    /// Can this process be completed?
    fn can_complete(&self) -> bool;
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
    Completed,
}
