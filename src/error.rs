use libpulse_binding::error::PAErr;
use std::fmt::Display;

#[derive(Debug)]
pub enum PulseCoreError {
    ConnectError,
}

impl Display for PulseCoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PulseCoreError::ConnectError => {
                write!(f, "Failed to connect to the server!")
            }
        }
    }
}

impl From<PAErr> for PulseCoreError {
    fn from(_: PAErr) -> Self {
        PulseCoreError::ConnectError
    }
}

impl std::error::Error for PulseCoreError {}
