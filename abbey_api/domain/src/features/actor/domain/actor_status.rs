use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ActorStatus {
    /// The [Actor][`crate::features::actor::domain::Actor`] is available.
    Available,
    /// The Actor is assigned to a [Process][`crate::features::process::domain::Process`].
    Assigned,
}

impl ActorStatus {
    /// Gets the locale code of the [`ActorStatus`].
    fn code(&self) -> &'static str {
        match self {
            Self::Available => "actor_status.available",
            Self::Assigned => "actor_status.assigned",
        }
    }
}

impl Display for ActorStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}
