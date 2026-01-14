pub struct TeamName(String);
#[derive(Debug)]
pub enum TeamNameValidationError {
    InvalidLength,
    InvalidCharacters,
}
impl TeamName {
    pub fn new(name: &str) -> Result<Self, TeamNameValidationError> {
        let len = name.chars().count();
        if len > 20 {
            return Err(TeamNameValidationError::InvalidLength);
        }
        if !name.chars().all(|c| c == ' ') {
            return Err(TeamNameValidationError::InvalidCharacters);
        }
        Ok(TeamName(name.to_string()))
    }
}
