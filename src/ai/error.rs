use shuru::ai::js_indexer::JsIndexerError;

#[derive(Debug, thiserror::Error)]
pub enum AiError {
    #[error("JS Indexer Error: {0}")]
    JsIndexerError(#[from] JsIndexerError),

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}
