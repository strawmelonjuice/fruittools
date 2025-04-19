use std::path::PathBuf;

#[derive(Debug)]
pub enum FruitToolsUnifiedErrorType {
    // General errors
    IOError(std::io::Error),
    UnreadableFile {
        file: PathBuf,
        io_error: std::io::Error,
        msg: String,
    },
    // Pulp-specific errors
    GleamManifestUnreadable,
    GleamManifestParseError,
    NoManifestRecognized,
}

