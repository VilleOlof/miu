use thiserror::Error;

#[derive(Debug, Error)]
pub enum MIUError {
    #[error("No items in Results")]
    EmptyResults,
    #[error("Failed to parse the weekly data: {0:?}")]
    FailedToParseWeekly(serde_json::Error),
    #[error("Failed to parse the scorebucket in weekly data: {0:?}")]
    FailedToParseScorebucket(serde_json::Error),
}
