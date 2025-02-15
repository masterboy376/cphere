use regex::Regex;

/// Validates that the given email is in a correct format.
/// This is a simple regex check and may not cover all valid emails.
pub fn validate_email(email: &str) -> bool {
    // The regex here is case-insensitive (`(?i)`) and checks for a common email pattern.
    let re = Regex::new(r"(?i)^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$").unwrap();
    re.is_match(email)
}

/// Validates that the username is at least 3 characters long
/// and consists of alphanumeric characters or underscores.
pub fn validate_username(username: &str) -> bool {
    if username.len() < 3 {
        return false;
    }
    username.chars().all(|c| c.is_alphanumeric() || c == '_')
}
