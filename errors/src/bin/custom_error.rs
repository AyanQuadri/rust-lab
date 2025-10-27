use core::fmt;

// Define specific network-related errors
#[derive(Debug)]
enum NetworkError {
    Disconnected,
    Timeout,
}

// Define more general custom errors that may wrap a network error
#[derive(Debug)]
enum CustomError {
    NotFound,
    PermissionDenied,
    ConnectionFailed,
    Network(NetworkError), // this allows embedding NetworkError inside CustomError
}

// Implement human-readable display text for NetworkError
impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NetworkError::Disconnected => write!(f, "Network Disconnected"),
            NetworkError::Timeout => write!(f, "Network Timeout"),
        }
    }
}

// Implement display text for CustomError
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CustomError::NotFound => write!(f, "Resource not found"),
            CustomError::PermissionDenied => write!(f, "Permission Denied"),
            CustomError::ConnectionFailed => write!(f, "Connection Failed"),
            // recursively call NetworkError’s display implementation
            CustomError::Network(e) => write!(f, "Network Error: {}", e),
        }
    }
}

// Mark both as valid Rust error types
impl std::error::Error for NetworkError {}
impl std::error::Error for CustomError {}

// Allow automatic conversion from NetworkError → CustomError
impl From<NetworkError> for CustomError {
    fn from(error: NetworkError) -> Self {
        CustomError::Network(error)
    }
}

// A function that simulates failing to connect to the network
fn connect_to_network() -> Result<(), NetworkError> {
    Err(NetworkError::Disconnected)
}

// A higher-level task function that uses `?` to automatically convert NetworkError to CustomError
fn perform_task() -> Result<(), CustomError> {
    connect_to_network()?; // `?` triggers conversion if error happens
    Ok(())
}

// Main: run the task and print the result
fn main() {
    match perform_task() {
        Ok(_) => println!("Task performed successfully!"),
        Err(e) => println!("Task failed: {}", e),
    }
}
