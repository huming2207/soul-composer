use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArmError {
    #[error("Section {0} not found, which is required to be present.")]
    StubSectionNotFound(String),
}