use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ActorStatus {
    Available,
    Assigned,
}

impl std::error::Error for ActorStatus {}

impl ActorStatus {
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
