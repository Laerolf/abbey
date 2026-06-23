#[derive(Debug)]
pub enum MonkErrorKind {
    /// Failed to get all Monks with the provided Monastery ID.
    GetAllByMonasteryId,
}
