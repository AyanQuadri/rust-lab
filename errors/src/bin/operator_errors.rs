// Function to read a username
fn read_username() -> Result<String, String> {
    // Return a username (success)
    Ok("john".to_string())
}

// Function to validate username
fn validate_username(username: &str) -> Result<(), String> {
    if username == "john" {
        // Valid username
        Ok(())
    } else {
        // Invalid username
        Err("Invalid username".to_string())
    }
}

// Main function with Result return type
fn main() -> Result<(), String> {
    // Try to read the username
    // If error occurs, return it directly
    let username = read_username()?;

    // Validate the username
    // If invalid, return Err early
    validate_username(&username)?;

    // Only runs if both succeeded
    println!("Username is valid!");

    // Program exited successfully
    Ok(())
}
