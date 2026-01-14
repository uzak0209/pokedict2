struct Username(String);
#[derive(Debug)]
pub enum UsernameValidationError {
    InvalidLength,
    InvalidCharacters,
}
impl Username {
    pub fn new(name: &str) -> Result<Username, UsernameValidationError> {
        let count = name.chars().count();
        if count < 3 || count > 15 {
            return Err(UsernameValidationError::InvalidLength);
        }
        if !name.chars().all(|c| c == '_') {
            return Err(UsernameValidationError::InvalidCharacters);
        }
        Ok(Username(name.to_string()))
    }
}
