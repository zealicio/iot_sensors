#[derive(Debug)]
pub enum AppError {
    InitializationFailed(String),
    I2CCommunicationError(String),
    SerialCommunicationError(String),
    DataParsingError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InitializationFailed(s) => write!(f, "Initialization Failed: {}", s),
            AppError::I2CCommunicationError(s) => write!(f, "I2C Communication Error: {}", s),
            AppError::SerialCommunicationError(s) => write!(f, "Serial communication Error: {}", s),
            AppError::DataParsingError(s) => write!(f, "Data Parsing Error: {}", s),
        }
    }
}

impl std::error::Error for AppError {}
