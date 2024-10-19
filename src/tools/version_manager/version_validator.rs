use shuru::error::VersionManagerError;

pub trait VersionValidator {
    fn validate_version(version: &str) -> Result<(), VersionManagerError>;
}
