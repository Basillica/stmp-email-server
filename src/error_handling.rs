use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SmtpError {
    kind: SmtpErrorKind,
    message: String,
}

#[derive(Debug)]
pub enum SmtpErrorKind {
    ConnectionError,
    ParseError,
    AuthenticationError,
    DeliveryError,
    EncryptionError,
    // Add more error kinds as needed
}

impl fmt::Display for SmtpErrorKind{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "some error")
    }
}

impl fmt::Display for SmtpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl Error for SmtpError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Implement source() if applicable
        None
    }
}