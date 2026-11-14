use std::fmt;

pub enum AuthErrorMessage {
    HashingError(String),
}

impl fmt::Display for AuthErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HashingError(error_string) => {
                write!(f, "Error while hashing password: {}", error_string)
            }
        }
    }
}
